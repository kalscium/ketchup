use ketchup::{node::Node, parser::Parser, token_info::{TokInfoOrCustom, TokenInfo}, Space, Span};
use logos::Logos;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    #[default]
    UnexpectedCharacter,
}

#[derive(Debug, Clone, Logos)]
#[logos(error = Error)]
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
    Neg,
    Pos,
}

fn token_informer<'a, Tokens>(token: Token, span: Span, double_space: bool) -> TokInfoOrCustom<Oper, Tokens, Error, Vec<Node<Oper>>> {
    use Token as T;
    use Oper as O;
    let (precedence, space, oper) = match (token, double_space) {
        (T::Number(x), _) => (0, Space::Zero, O::Num(x)),
        (T::Plus, false) => (1, Space::One, O::Pos),
        (T::Minus, false) => (1, Space::One, O::Neg),

        (T::Star, _) => (2, Space::Two, O::Mul),
        (T::Slash, _) => (2, Space::Two, O::Div),
        (T::Plus, true) => (3, Space::Two, O::Add),
        (T::Minus, true) => (3, Space::Two, O::Sub),
    };

    TokInfoOrCustom::TokenInfo(TokenInfo {
        oper,
        span,
        space,
        precedence,
    })
}

fn main() {
    const SRC: &'static str = "-1 + 2 * -3 - 4 / +8 + 27";
    
    let lexer = Token::lexer(SRC);
    let parser = Parser::<Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(lexer.spanned(), token_informer);
    let asa = match parser.parse() {
        Ok(x) => x,
        Err(e) => {
            e.iter().for_each(|x| println!("{x:?}"));
            panic!("an error occurred");
        },
    };

    // println!("{asa:?}");
    println!("{:?}", asa.iter().map(|node| &node.oper).collect::<Vec<_>>());
}
