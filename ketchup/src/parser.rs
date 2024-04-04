use std::cmp::Ordering;

use crate::{asa, node::{Node, TokenInfo}, Span};

/// Parser that generates the nodes within an `ASA`
#[derive(Debug)]
pub struct Parser<Token, Tokens, ASA, TokenInformer, Error>
where
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
    pub fn parse(mut self) {
        for token in self.tokens {
            // grab the current token and token information
            let (token, span) = token;
            let token = token.unwrap(); // we're not gonna deal with errors yet as this is a mere PoC
            let tok_info = (self.tok_informer)(&token, span);

            // if the `ASA` is empty simply push to it, otherwise check the precedence
            let pointer = match self.pointer {
                Some(x) => x,
                None => {
                    self.asa.push(Node::new(token, tok_info.span, tok_info.space, None, tok_info.precedence));
                    self.pointer = Some(0); // sets pointer to the pushed node
                    continue
                }
            };

            // grab pointer information & compare precedence
            let pointed = &self.asa.get(pointer);
            match pointed.precedence.cmp(&tok_info.precedence) {
                // become owned by the pointed
                Ordering::Less => {
                    self.asa.push(Node::new(token, tok_info.span, tok_info.space, Some(pointer), tok_info.precedence));
                    self.pointer = Some(self.asa.len() - 1);
                },
                _ => todo!(),
            }
        }
    }
}
