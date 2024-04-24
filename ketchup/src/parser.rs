use core::fmt::Debug;
use crate::{asa, error::Error as KError, map, node::Node, token_info::{TokInfoOrCustom, TokenInfo}, Space, Span};

/// Parser that generates the nodes within an `ASA`
#[derive(Debug)]
pub struct Parser<'a, Token, Oper, Tokens, ASA, TokenInformer, Error>
where
    Token: PartialEq,
    Oper: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Oper = Oper>,
    TokenInformer: Fn(Token, Span, bool) -> TokInfoOrCustom<Oper, Token, Tokens, Error>,
{
    /// a pointer to a function that provides information about a token,
    /// it takes in the `token`, `span` of that token and also if a `double_space` is allowed and returns the `tok_info`
    tok_informer: TokenInformer,
    /// the iterator that provides the tokens for the parser
    tokens: &'a mut Tokens,
    /// a token that caps off the iterator (that gets ignored)
    eof: Option<Token>,
    /// the internal `ASA`
    asa: ASA,
}

impl<'a, Token, Oper, Tokens, ASA, TokenInformer, Error> Parser<'a, Token, Oper, Tokens, ASA, TokenInformer, Error>
where
    Token: PartialEq,
    Oper: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Oper = Oper>,
    TokenInformer: Fn(Token, Span, bool) -> TokInfoOrCustom<Oper, Token, Tokens, Error>,
{
    /// initialises a new parser with the provided tokens and token_info
    #[inline]
    pub fn new(tokens: &'a mut Tokens, eof: Option<Token>, tok_informer: TokenInformer) -> Self {
        Self {
            tokens,
            eof,
            tok_informer,
            asa: ASA::default(),
        }
    }

    /// returns the current token & token information
    #[inline]
    #[allow(clippy::type_complexity)]
    fn get_next_tok(&mut self, double_space: bool) -> Result<Option<TokInfoOrCustom<Oper, Token, Tokens, Error>>, KError<Token, Error>> {
        let (token, span) = match self.tokens.next() {
            Some(x) => x,
            None => {
                // check that the eof has been reached
                return if let Some(eof) = self.eof.take() {
                    Err(KError::ExpectedEOF {
                        eof,
                    })
                } else {
                    Ok(None)
                }
            },
        };

        let token = token.map_err(|e| KError::Other(span.clone(), e))?; // lexer may throw errors
        if Some(&token) == self.eof.as_ref() { return Ok(None) }; // check for lexer eof
        let tok_info = (self.tok_informer)(token, span, double_space);

        Ok(Some(tok_info))
    }

    /// safely inserts a node into the `ASA` while following `ketchup`'s rules
    #[inline]
    fn safe_insert(&mut self, mut pointer: usize, node: Node<Oper>, double_space: bool) -> Result<usize, KError<Token, Error>> {
        let pointed = self.asa.get(pointer).info_mut();

        // make sure that there aren't any double spaces next to each other
        if pointed.space && double_space {
            return Err(
                KError::DoubleSpaceConflict {
                    span: node.take_info().span
                }
            );
        }

        
        // compare against the last pointed node
        if node.info().precedence < pointed.precedence || !node.info().space {
            // become owned by the pointed

            // check if it's violating double-space rules
            if pointed.space && double_space {
                return Err(
                    KError::DoubleSpaceConflict {
                        span: node.take_info().span
                    }
                );
            }

            // check if there is enough space
            if !pointed.space {
                return Err(
                    KError::UnexpectedOper(
                        node.take_info().span
                    )
                );
            }

            // push to the `ASA` and update variables
            pointed.space = false;
            self.asa.push(map(node, |node| node.info_mut().parent = Some(pointer)));
            pointer = self.asa.len() - 1;
        } else {
            // take ownership of the pointed

            let mut opt_parent_idx = Some(pointer);
            loop {
                let parent_idx = match opt_parent_idx {
                    Some(x) => x,
                    // if at the start of the `ASA` just insert to the start
                    None => {
                        self.asa.get(pointer).info_mut().parent = Some(0); // update pointed parent
                        self.asa.insert(0, node);
                        pointer = 0;

                        break
                    },
                };

                let parent = self.asa.get(parent_idx).info();
                if parent.precedence > node.info().precedence {
                    // update the pointed's owner
                    self.asa.get(pointer).info_mut().parent = Some(pointer);
                    // replace the pointed and own it
                    self.asa.insert(pointer, map(node, |node| node.info_mut().parent = Some(parent_idx))); // `-1` space cause it's already owning the pointed off the bat
                    break
                }

                pointer = parent_idx;
                opt_parent_idx = parent.parent; // move on to the parent's parent (grandparent)
            }
        }

        Ok(pointer)
    }

    /// parses only one token (doesn't consume parser) and returns the current pointer
    #[inline]
    fn parse_once(&mut self, pointer: usize, tok_info: TokenInfo<Oper>) -> Result<usize, KError<Token, Error>> {
        self.safe_insert(pointer, Node::new_node(
            tok_info.oper,
            tok_info.span,
            None,
            tok_info.precedence,
            tok_info.space != Space::Zero,
        ), tok_info.space == Space::Two)
    }

    /// parses the **first** token/node of the `ASA` and returns the pointer
    #[inline]
    fn parse_first_tok(&mut self, node: Node<Oper>, double_space: bool) -> Result<usize, KError<Token, Error>> {
        // check if the token's space is valid at the start
        // (a double-spaced token cannot be at the start of the ASA with no inputs)
        if double_space {
            return Err(
                KError::DoubleSpaceConflict {
                    span: node.take_info().span,
                }
            );
        }
    
        // push the first node onto the `ASA` to be the first parent
        self.asa.push(node);
    
        Ok(0) // set the pointer to the first node
    }
    
    /// comsumes the parser, parses and generates the `ASA`
    #[inline]
    pub fn parse(mut self) -> Result<ASA, Vec<KError<Token, Error>>> {
        let mut pointer = 'pointer: {
            // get token & token info, otherwise return empty `ASA`
            let tok_info = match self.get_next_tok(false).map_err(|e| vec![e])? {
                Some(TokInfoOrCustom::TokenInfo(x)) => x,
                Some(TokInfoOrCustom::Custom(f)) => {
                    let (node_info, asa, double_space) = f(self.tokens, None)?;
                    break 'pointer self.parse_first_tok(Node::Scoped(asa, node_info), double_space).map_err(|e| vec![e])?;
                },
                None => return Ok(self.asa), // `ASA` is empty
            };

            self.parse_first_tok(Node::new_node(
                tok_info.oper,
                tok_info.span,
                None,
                tok_info.precedence,
                tok_info.space != Space::Zero,
            ), tok_info.space == Space::Two).map_err(|e| vec![e])?
        };

        loop { // would use an iterator-
            let double_space = !self.asa.get(pointer).info().space;
            let tok_info = match self.get_next_tok(double_space).map_err(|e| vec![e])? { // -but can't
                Some(TokInfoOrCustom::TokenInfo(x)) => x,
                Some(TokInfoOrCustom::Custom(f)) => {
                    let (node_info, asa, double_space) = f(self.tokens, Some(pointer))?;
                    pointer = self.safe_insert(pointer, Node::Scoped(asa, node_info), double_space).map_err(|e| vec![e])?;
                    continue
                },
                None => break,
            };

            pointer = self.parse_once(pointer, tok_info).map_err(|e| vec![e])?;
        }

        // if an operation or it's parent has a missing input then throw an error
        let pointed = self.asa.get(pointer).info();
        if pointed.space {
            return Err(vec![
                KError::ExpectedOper {
                    span: pointed.span.start + 2..pointed.span.end + 2, // replace with the actual span of the EOF
                    precedence: pointed.precedence,
                }
            ]);
        }
        match pointed.parent.map(|x| self.asa.get(x)) {
            Some(parent) if parent.info().space => {
                return Err(vec![
                    KError::ExpectedOper {
                        span: parent.info().span.start + 2..parent.info().span.end + 2, // replace with the actual span of the EOF
                        precedence: parent.info().precedence,
                    }
                ])
            },
            _ => (),
        }

        // return the completed **valid** ASA
        Ok(self.asa)
    }
}
