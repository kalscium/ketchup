use ketchup::{node::Node, parser::Parser, OperInfo, Space};
use logos::{Logos, SpannedIter};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    #[default]
    UnexpectedCharacter,
    EmptyBraces,
}

#[derive(Debug, Clone, Logos, PartialEq)]
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
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
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
    Scope(Vec<Oper>),
}

fn visit_node(idx: usize, asa: &Vec<Node<Oper>>) -> (usize, f64) {
    let oper = &asa[idx].oper;

    match oper {
        Oper::Num(x) => (idx, *x),

        Oper::Pos => visit_node(idx+1, asa),
        Oper::Neg => {
            let (idx, x) = visit_node(idx+1, asa);
            (idx, -x)
        },

        Oper::Add => {
            let (idx, x) = visit_node(idx+1, asa);
            let (idx, y) = visit_node(idx+1, asa);

            (idx, x + y)
        },

        Oper::Sub => {
            let (idx, x) = visit_node(idx+1, asa);
            let (idx, y) = visit_node(idx+1, asa);

            (idx, x - y)
        },

        Oper::Mul => {
            let (idx, x) = visit_node(idx+1, asa);
            let (idx, y) = visit_node(idx+1, asa);

            (idx, x * y)
        },

        Oper::Div => {
            let (idx, x) = visit_node(idx+1, asa);
            let (idx, y) = visit_node(idx+1, asa);

            (idx, x / y)
        },

        _ => todo!(),
    }
}

fn oper_generator(token: Token, tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> OperInfo<Oper> {
    use Token as T;
    use Oper as O;

    let (precedence, space, oper) = match (token, double_space) {
        // no space
        (T::Number(x), _) => (0, Space::None, O::Num(x)),

        // single space
        (T::Plus, false) => (1, Space::Single, O::Pos),
        (T::Minus, false) => (1, Space::Single, O::Neg),

        // double space
        (T::Plus, true) => (3, Space::Double, O::Add),
        (T::Minus, true) => (3, Space::Double, O::Sub),
        (T::Star, _) => (2, Space::Double, O::Mul),
        (T::Slash, _) => (2, Space::Double, O::Div),
        
        _ => todo!(),
    };

    OperInfo {
        oper,
        space,
        precedence,
    }
}

fn main() {
    const SRC: &str = "1- + 2 * 2";

    let mut lexer = Token::lexer(SRC).spanned();
    let parser = Parser::<'_, Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut lexer, None, oper_generator);

    let asa = parser.parse().unwrap();
    println!("{asa:?}");

    if asa.is_empty() { return };

    let out = visit_node(0, &asa);
    println!("result: {out:?}");
}
