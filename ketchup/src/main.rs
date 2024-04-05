use std::cmp::Ordering;
use ketchup::{asa::{self, ASA}, node::{Node, TokenInfo}, parser::Parser, Span};
use logos::{Logos, Lexer};

#[derive(Debug, Clone, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
}

fn token_informer(token: &Token, span: Span) -> TokenInfo {
    use Token as T;
    let (precedence, space) = match token {
        T::Number(_) => (0, 0),
        T::Mul => (1, 2),
        T::Div => (1, 2),
        T::Plus => (2, 2),
        T::Minus => (2, 2),
    };

    TokenInfo {
        span,
        space,
        precedence,
    }
}

fn main() {
    const SRC: &'static str = "1 + 2 * 3 / 4 / 8 + 27";
    
    let lexer = Token::lexer(SRC);
    let mut parser = Parser::<Token, _, Vec<Node<Token>>, _, ()>::new(lexer.spanned(), token_informer);
    let asa = parser.parse();

    println!("{:?}", asa.iter().map(|node| &node.token).collect::<Vec<_>>());
}
