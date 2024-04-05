use core::{cmp::Ordering, fmt::Debug};
use crate::{asa, node::{Node, TokenInfo}, Span};

/// Parser that generates the nodes within an `ASA`
#[derive(Debug)]
pub struct Parser<Token, Tokens, ASA, TokenInformer, Error>
where
    Token: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Token = Token>,
    TokenInformer: Fn(&Token, Span) -> TokenInfo,
{
    /// a pointer to a function that provides information about a token
    tok_informer: TokenInformer,
    /// the iterator that provides the tokens for the parser
    tokens: Tokens,
    /// the internal `ASA`
    asa: ASA,
    /// a pointer to the current node in the `ASA`
    pointer: Option<usize>,
    /// the amount of space remaining below the current node
    space: u8,
}

impl<Token, Tokens, ASA, TokenInformer, Error> Parser<Token, Tokens, ASA, TokenInformer, Error>
where
    Token: Debug,
    Error: std::fmt::Debug,
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
    ASA: asa::ASA<Token = Token>,
    TokenInformer: Fn(&Token, Span) -> TokenInfo,
{
    /// initialises a new parser with the provided tokens and token_info
    #[inline]
    pub fn new(tokens: Tokens, tok_informer: TokenInformer) -> Self {
        Self {
            tokens,
            tok_informer,
            asa: ASA::default(),
            pointer: None,
            space: 0,
        }
    }
    
    /// comsumes the parser, parses and generates the `ASA`
    #[inline]
    pub fn parse(mut self) -> ASA {
        for token in self.tokens {
            // grab the current token and token information
            let (token, span) = token;
            let token = token.unwrap(); // we're not gonna deal with errors yet as this is a mere PoC
            let tok_info = (self.tok_informer)(&token, span);

            // if the `ASA` is empty simply push to it, otherwise check the precedence
            let mut pointer = match self.pointer {
                Some(x) => x,
                None => {
                    self.asa.push(Node::new(token, tok_info.span, tok_info.space, None, tok_info.precedence));
                    self.pointer = Some(0); // sets pointer to the pushed node
                    continue
                }
            };

            loop { // to allow for recursive comparing against parents of nodes
                // grab pointer information & compare precedence
                let pointed = &self.asa.get(pointer);
                match tok_info.precedence.cmp(&pointed.precedence) {
                    // become owned by the pointed
                    Ordering::Less => {
                        self.asa.push(Node::new(token, tok_info.span, tok_info.space, Some(pointer), tok_info.precedence));
                        self.pointer = Some(self.asa.len() - 1);
                        break // no need for recursion
                    },
                    Ordering::Equal | Ordering::Greater => {
                        match pointed.parent {
                            None => {
                                self.asa.insert(0, Node::new(token, tok_info.span, tok_info.space, None, tok_info.precedence));
                                self.pointer = Some(0);
                                break // no need for recursion as you're already at the start of the `ASA`
                            },
                            Some(parent) => {
                                pointer = parent;
                                continue // recursion
                            },
                        }
                    },
                }
            }
        }

        // return the built `ASA`
        return self.asa;
    }
}
