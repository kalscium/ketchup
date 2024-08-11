use ariadne::{Color, Label, Report, ReportKind, Source};
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

fn oper_generator(token: Token, _tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> OperInfo<Oper> {
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
    // source to parse
    const SRC: &str = "1 + 45 - 2 * 3";

    let mut lexer = Token::lexer(SRC).spanned();
    let parser = Parser::<'_, Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut lexer, None, oper_generator);

    // handle errors
    let asa = match parser.parse() {
        Ok(asa) => asa,
        Err(err) => {
            Report::build(ReportKind::Error, "sample.foo", 12)
                .with_message(format!("{err:?}"))
                .with_label(
                    Label::new(("sample.foo", err.span().clone()))
                        .with_message("occured here")
                        .with_color(Color::Red)
                )
                .with_note("errors will look a bit funny cuz i'm too lazy to put in custom messages")
                .finish()
                .eprint(("sample.foo", Source::from(SRC)))
                .unwrap();
            panic!("an error occured");
        },
    };

    // print abstract syntax array
    println!("{asa:?}");

    // don't visit the ASA if it's empty
    if asa.is_empty() { return };

    // visit / interpret the ASA
    let out = visit_node(0, &asa).1;
    println!("result: {out:?}");
}
