use std::fmt::Debug;
use crate::{error::KError, node::{Node, NodeInfo}, OperInfo, Space, Span};

#[derive(Debug)]
pub struct Parser<'a, Token, Oper, Tokens, ASA, OperGen, Error>
where
    Token: PartialEq,
    Oper: Debug,
    Error: Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: crate::asa::ASA<Oper = Oper>,
    OperGen: Fn(Token, &mut Tokens, bool) -> Result<OperInfo<Oper>, Vec<KError<Token, Error>>>,
{
    /// A function that generates an operation from a token-iterator,
    /// *(effectively a mini context-less parser)*
    ///
    /// The three parameters to the function are the current token, a mutable reference to the token-iterator, and a boolean of if the oper is allowed to be double-spaced (`_ x _`) or not
    oper_gen: OperGen,
    /// The iterator that provides the tokens for the parser
    tokens: &'a mut Tokens,
    /// A token that caps off the iterator (and gets ignored, eg `)`)
    eof: Option<Token>,
    /// The internal `ASA`
    asa: ASA,
}

impl<'a, Token, Oper, Tokens, ASA, OperGen, Error> Parser<'a, Token, Oper, Tokens, ASA, OperGen, Error>
where
    Token: PartialEq,
    Oper: Debug,
    Error: Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: crate::asa::ASA<Oper = Oper>,
    OperGen: Fn(Token, &mut Tokens, bool) -> Result<OperInfo<Oper>, Vec<KError<Token, Error>>>,
{
    /// Initialises a new parser with the provided token iterator, optional EOF token, and operation generator
    #[inline]
    pub fn new(tokens: &'a mut Tokens, eof: Option<Token>, oper_gen: OperGen) -> Self {
        Self {
            tokens,
            eof,
            oper_gen,
            asa: ASA::default(),
        }
    }

    /// Returns the current oper information
    #[allow(clippy::type_complexity)]
    fn parse_next_oper(&mut self, double_space: bool) -> Result<Option<OperInfo<Oper>>, Vec<KError<Token, Error>>> {
        let (token, span) = match self.tokens.next() {
            Some((token, span)) => (token, span),
            // if there are no more tokens left in the iterator
            None => {
                // if the asa is empty and there is no more tokens, then the token iterator must be empty aswell and therefore this must shortcircuit to prevent subtraction overflow errors
                if self.asa.is_empty() {
                    return Ok(None);
                }
                
                // throw error if eof has not been reached
                // also make sure there is an eof to expect in the first place
                return if let Some(eof) = self.eof.take() {
                    let span = self.asa.get(self.asa.len()-1).info.span.end;
                    Err(vec![
                        KError::ExpectedEOF {
                            eof,
                            span: span..span,
                        }
                    ])
                } else {
                    Ok(None)
                };
            },
        };

        let token = token.map_err(|e| KError::Other(span.clone(), e)).map_err(|e| vec![e])?; // lexer may throw errors

        // check for lexer eof
        if Some(&token) == self.eof.as_ref() {
            return Ok(None);
        }

        let oper_info = (self.oper_gen)(token, self.tokens, double_space)?;
        Ok(Some(oper_info))
    }

    /// Safely inserts an oper into the `ASA` while following `ketchup`'s rules
    ///
    /// Also returns the current pointer
    fn safe_insert(&mut self, mut pointer: usize, oper_info: OperInfo<Oper>) -> Result<usize, KError<Token, Error>> {
        let pointed = &mut self.asa.get(pointer).info;

        // make sure that there aren't any double spaces next to each other
        if pointed.space && oper_info.space == Space::Double {
            return Err(
                KError::DoubleSpaceConflict {
                    span: oper_info.span,
                }
            );
        }

        // compare against the last pointed node
        if oper_info.precedence < pointed.precedence || oper_info.space == Space::None {
            // become owned by the pointed

            // check if there is enough space for another node input
            if !pointed.space {
                return Err(
                    KError::UnexpectedOper(
                        oper_info.span
                    )
                );
            }

            // push to the `ASA` and update variables
            pointed.space = false;
            self.asa.push(Node {
                oper: oper_info.oper,
                info: NodeInfo {
                    span: oper_info.span,
                    parent: Some(pointer),
                    precedence: oper_info.precedence,
                    space: oper_info.space != Space::None,
                },
            });
            pointer = self.asa.len()-1;
        } else {
            // take owernership of the pointed recursively

            // make sure that the oper has enough space to own the pointed
            if oper_info.space == Space::None || (pointed.precedence == 0 && oper_info.space == Space::Single) { // single-spaced oper is not allowed due to the parser being left-aligned
                return Err(KError::UnexpectedOper(oper_info.span));
            }

            let mut opt_parent_idx = Some(pointer);
            loop {
                let parent_idx = match opt_parent_idx {
                    Some(idx) => idx,
                    None => {
                        self.asa.get(pointer).info.parent = Some(0);
                        self.asa.insert(0, Node {
                            oper: oper_info.oper,
                            info: NodeInfo {
                                span: oper_info.span,
                                parent: None,
                                precedence: oper_info.precedence,
                                space: oper_info.space != Space::None,
                            },
                        });

                        pointer = 0;
                        break
                    },
                };

                let parent = &self.asa.get(parent_idx).info;
                if parent.precedence > oper_info.precedence {
                    // update the pointed's owner
                    self.asa.get(pointer).info.parent = Some(pointer);

                    // replace the pointed and own it
                    self.asa.insert(pointer, Node {
                        oper: oper_info.oper,
                        info: NodeInfo {
                            span: oper_info.span,
                            parent: Some(parent_idx),
                            precedence: oper_info.precedence,
                            space: oper_info.space != Space::None, // `-1` space cuz it's already owning the pointed off the bat
                        },
                    });

                    break
                }

                pointer = parent_idx;
                opt_parent_idx = parent.parent;
            }
        }
        
        Ok(pointer)
    }

    /// Parses the **first** token/oper of the `ASA` and returns the pointer
    fn parse_first_tok(&mut self, oper_info: OperInfo<Oper>) -> Result<usize, KError<Token, Error>> {
        // check if the oper's space is valid at the start
        // (a double-spaced oper cannot be at the start of the ASA (due to an unfullfied input `? x _`))
        if oper_info.space == Space::Double {
            return Err(
                KError::DoubleSpaceConflict {
                    span: oper_info.span,
                }
            );
        }

        // push the first node onto the ASA to be the first parent
        self.asa.push(Node {
            oper: oper_info.oper,
            info: NodeInfo {
                span: oper_info.span,
                parent: None,
                precedence: oper_info.precedence,
                space: oper_info.space != Space::None,
            },
        });

        Ok(0) // first node is the pointer
    }

    pub fn parse(mut self) -> Result<ASA, Vec<KError<Token, Error>>> {
        let mut pointer = {
            let oper_info = match self.parse_next_oper(false)? {
                Some(info) => info,
                None => return Ok(self.asa), // there are no tokens to parse at all
            };

            self.parse_first_tok(oper_info).map_err(|e| vec![e])?
        };

        // iterate over and parse the rest of the tokens
        loop {
            let double_space = !self.asa.get(pointer).info.space; // if the next oper should be allowed to have a double-space
            let oper_info = match self.parse_next_oper(double_space)? {
                Some(info) => info,
                None => break,
            };

            pointer = self.safe_insert(pointer, oper_info).map_err(|e| vec![e])?;
        }
        
        // if a node has a missing input then throw an error
        let pointed = &self.asa.get(pointer).info;
        if pointed.space {
            return Err(vec![
                KError::ExpectedOper {
                    span: pointed.span.end..pointed.span.end, // replace with the actual span of the EOF
                    precedence: pointed.precedence,
                }
            ]);
        }

        // if a node's parent has a missing input then throw an error
        match pointed.parent.map(|x| self.asa.get(x)) {
            Some(parent) if parent.info.space => {
                return Err(vec![
                    KError::ExpectedOper {
                        span: parent.info.span.end..parent.info.span.end, // replace with the actual span of the EOF
                        precedence: parent.info.precedence,
                    }
                ])
            }
            _ => (),
        }

        // return the completed **valid** ASA
        Ok(self.asa)
    }
}
