use core::fmt::Debug;
use crate::{asa, error::Error as KError, node::Node, token_info::{TokenInfo, TokInfoOrCustom}, Space, Span};

/// Parser that generates the nodes within an `ASA`
#[derive(Debug)]
pub struct Parser<Token, Oper, Tokens, ASA, TokenInformer, Error>
where
    Oper: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Oper = Oper>,
    TokenInformer: Fn(Token, Span) -> TokInfoOrCustom<Oper, Tokens, Error, ASA>,
{
    /// a pointer to a function that provides information about a token
    tok_informer: TokenInformer,
    /// the iterator that provides the tokens for the parser
    tokens: Tokens,
    /// the internal `ASA`
    asa: ASA,
}

impl<Token, Oper, Tokens, ASA, TokenInformer, Error> Parser<Token, Oper, Tokens, ASA, TokenInformer, Error>
where
    Oper: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Oper = Oper>,
    TokenInformer: Fn(Token, Span) -> TokInfoOrCustom<Oper, Tokens, Error, ASA>,
{
    /// initialises a new parser with the provided tokens and token_info
    #[inline]
    pub fn new(tokens: Tokens, tok_informer: TokenInformer) -> Self {
        Self {
            tokens,
            tok_informer,
            asa: ASA::default(),
        }
    }

    /// returns the current token & token information
    #[inline]
    fn get_next_tok(&mut self) -> Result<Option<TokInfoOrCustom<Oper, Tokens, Error, ASA>>, KError<Error>> {
        let (token, span) = match self.tokens.next() {
            Some(x) => x,
            None => return Ok(None),
        };
        let token = token.map_err(|e| KError::Other(span.clone(), e))?;
        let tok_info = (self.tok_informer)(token, span);

        Ok(Some(tok_info))
    }

    /// parses only one token (doesn't consume parser) and returns the current pointer
    #[inline]
    fn parse_once(&mut self, mut pointer: usize, tok_info: TokenInfo<Oper>) -> Result<usize, KError<Error>> {
        // compare against the last pointed node
        let pointed = self.asa.get(pointer);
        if tok_info.precedence < pointed.precedence || tok_info.space == Space::Zero {
            // become owned by the pointed

            // check if there is enough space
            if !pointed.space {
                return Err(
                    KError::UnexpectedOper(
                        tok_info.span
                    )
                );
            }

            // check if it's violating double-space rules
            if pointed.space && tok_info.space == Space::Two {
                return Err(
                    KError::DoubleSpaceConflict {
                        span: tok_info.span
                    }
                );
            }

            // push to the `ASA` and update variables
            pointed.space = false;
            self.asa.push(Node::new(tok_info.oper, tok_info.span, Some(pointer), tok_info.precedence, tok_info.space != Space::Zero));
            pointer = self.asa.len() - 1;
        } else {
            // take ownership of the pointed

            let mut opt_parent_idx = Some(pointer);
            loop {
                let parent_idx = match opt_parent_idx {
                    Some(x) => x,
                    // if at the start of the `ASA` just insert to the start
                    None => {
                        self.asa.get(pointer).parent = Some(0); // update pointed parent
                        self.asa.insert(0, Node::new(tok_info.oper, tok_info.span, None, tok_info.precedence, true));
                        pointer = 0;

                        break
                    },
                };

                let parent = self.asa.get(parent_idx);
                if parent.precedence > tok_info.precedence {
                    // update the pointed's owner
                    self.asa.get(pointer).parent = Some(pointer);
                    // replace the pointed and own it
                    self.asa.insert(pointer, Node::new(tok_info.oper, tok_info.span, Some(parent_idx), tok_info.precedence, true)); // `-1` space cause it's already owning the pointed off the bat
                    break
                }

                // check if the parent has space (you cannot own a node with non-zero space)
                if parent.space {
                    return Err(
                        KError::DoubleSpaceConflict {
                            span: tok_info.span
                        }
                    );
                }

                pointer = parent_idx;
                opt_parent_idx = parent.parent; // move on to the parent's parent (grandparent)
            }
        }

        Ok(pointer)
    }

    /// parses the **first** token/node of the `ASA` and returns the pointer
    #[inline]
    fn parse_first_tok(&mut self, tok_info: TokenInfo<Oper>) -> Result<usize, KError<Error>> {
        // check if the token's space is valid at the start
        // (a double-spaced token cannot be at the start of the ASA with no inputs)
        if let Space::Two = tok_info.space {
            return Err(
                KError::DoubleSpaceConflict {
                    span: tok_info.span,
                }
            );
        }
    
        // push the first node onto the `ASA` to be the first parent
        self.asa.push(Node::new(tok_info.oper, tok_info.span, None, tok_info.precedence, tok_info.space != Space::Zero));
    
        Ok(0) // set the pointer to the first node
    }
    
    /// comsumes the parser, parses and generates the `ASA`
    #[inline]
    pub fn parse(mut self) -> Result<ASA, Vec<KError<Error>>> {
        let mut pointer = {
            let tok_info = loop {
                // get token & token info, otherwise return empty `ASA`
                match self.get_next_tok().map_err(|e| vec![e])? {
                    Some(TokInfoOrCustom::TokenInfo(x)) => break x,
                    Some(TokInfoOrCustom::Custom(f)) => {
                        f(&mut self.tokens, &mut self.asa)?;
                    },
                    None => return Ok(self.asa), // `ASA` is empty
                };
            };

            self.parse_first_tok(tok_info).map_err(|e| vec![e])?
        };

        loop { // would use an iterator-
            let tok_info = match self.get_next_tok().map_err(|e| vec![e])? { // -but can't
                Some(TokInfoOrCustom::TokenInfo(x)) => x,
                Some(TokInfoOrCustom::Custom(f)) => {
                    f(&mut self.tokens, &mut self.asa)?;
                    continue;
                },
                None => break,
            };

            pointer = self.parse_once(pointer, tok_info).map_err(|e| vec![e])?;
        }

        // if an operation or it's parent has a missing input then throw an error
        let pointed = self.asa.get(pointer);
        if pointed.space {
            return Err(vec![
                KError::ExpectedOper {
                    span: pointed.span.start + 2..pointed.span.end + 2, // replace with the actual span of the EOF
                    precedence: pointed.precedence,
                }
            ]);
        }
        match pointed.parent.map(|x| self.asa.get(x)) {
            Some(parent) if parent.space => {
                return Err(vec![
                    KError::ExpectedOper {
                        span: parent.span.start + 2..parent.span.end + 2, // replace with the actual span of the EOF
                        precedence: parent.precedence,
                    }
                ])
            },
            _ => (),
        }

        // return the completed **valid** ASA
        Ok(self.asa)
    }
}
