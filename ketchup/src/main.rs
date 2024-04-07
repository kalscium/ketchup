use std::cmp::Ordering;
use ketchup::{asa::{self, ASA}, node::{Node, TokenInfo}, parser::Parser, Span};
use logos::{Logos, Lexer};

#[derive(Debug, Clone, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[regex(r"(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
}

#[derive(Debug, Clone)]
pub enum Oper {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
}

fn token_informer(token: Token, span: Span) -> (Oper, TokenInfo) {
    use Token as T;
    use Oper as O;
    let (precedence, space, oper) = match token {
        T::Number(x) => (0, 0, O::Num(x)),
        T::Star => (1, 2, O::Mul),
        T::Slash => (1, 2, O::Div),
        T::Plus => (2, 2, O::Add),
        T::Minus => (2, 2, O::Sub),
    };

    (oper, TokenInfo {
        span,
        space,
        precedence,
    })
}

fn main() {
    const SRC: &'static str = "1 + 2 * 3 / 4 / 8 + 27";
    
    let lexer = Token::lexer(SRC);
    let parser = Parser::<Token, Oper, _, Vec<Node<Oper>>, _, ()>::new(lexer.spanned(), token_informer);
    let asa = parser.parse();

    println!("{:?}", asa.iter().map(|node| &node.token).collect::<Vec<_>>());
}
