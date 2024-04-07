use core::fmt::Debug;
use crate::{asa, error::Error as KError, node::Node, token_info::TokInfoOrCustom, Span};

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
    fn get_next_tok(&mut self) -> Option<TokInfoOrCustom<Oper, Tokens, Error, ASA>> {
        let (token, span) = self.tokens.next()?;
        let token = token.unwrap(); // we're not gonna deal with errors yet as this is a mere PoC
        let tok_info = (self.tok_informer)(token, span);

        Some(tok_info)
    }
    
    /// comsumes the parser, parses and generates the `ASA`
    #[inline]
    pub fn parse(mut self) -> Result<ASA, Vec<KError<Error>>> {
        let mut space = 0;
        let mut pointer = {
            let tok_info = loop {
                // get token & token info, otherwise return empty `ASA`
                match self.get_next_tok() {
                    Some(TokInfoOrCustom::TokenInfo(x)) => break x,
                    Some(TokInfoOrCustom::Custom(f)) => {
                        f(&mut self.tokens, &mut self.asa)?;
                    },
                    None => return Ok(self.asa), // `ASA` is empty
                };
            };

            // push the first node onto the `ASA` to be the first parent
            self.asa.push(Node::new(tok_info.oper, tok_info.span, None, tok_info.precedence));
            
            0 // set the pointer to the first node
        };

        loop { // would use an iterator-
            let tok_info = match self.get_next_tok() { // -but can't
                Some(TokInfoOrCustom::TokenInfo(x)) => x,
                Some(TokInfoOrCustom::Custom(f)) => {
                    f(&mut self.tokens, &mut self.asa)?;
                    continue;
                },
                None => break,
            };

            // compare against the last pointed node
            let pointed = self.asa.get(pointer);
            if tok_info.precedence < pointed.precedence {
                // become owned by the pointed
                self.asa.push(Node::new(tok_info.oper, tok_info.span, Some(pointer), tok_info.precedence));
                pointer = self.asa.len() - 1;
            } else {
                // take ownership of the pointed

                let mut opt_parent_idx = pointed.parent.clone(); // clone is fine; `Option<usize>`
                loop {
                    let parent_idx = match opt_parent_idx {
                        Some(x) => x,
                        // if at the start of the `ASA` just insert to the start
                        None => {
                            self.asa.insert(0, Node::new(tok_info.oper, tok_info.span, None, tok_info.precedence));
                            pointer = 0;
                            break
                        },
                    };

                    let parent = self.asa.get(parent_idx);
                    if parent.precedence > tok_info.precedence {
                        // replace the pointed and own it
                        self.asa.insert(pointer, Node::new(tok_info.oper, tok_info.span, Some(parent_idx), tok_info.precedence));
                        break
                    }

                    pointer = parent_idx;
                    opt_parent_idx = parent.parent; // move on to the parent's parent (grandparent)
                }
            }
        }

        Ok(self.asa)
    }
}
