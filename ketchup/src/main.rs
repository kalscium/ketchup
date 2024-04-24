use ketchup::{error::Error as KError, node::{Node, NodeInfo}, parser::Parser, token_info::{TokInfoOrCustom, TokenInfo}, Space, Span};
use logos::{Logos, SpannedIter};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    #[default]
    UnexpectedCharacter,
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
}

#[inline]
fn visit_oper(idx: usize, asa: &Vec<Node<Oper>>) -> (usize, f64) {
    let oper = match &asa[idx] {
        Node::Node(x) => &x.oper,
        Node::Scoped(asa, _) => return (idx, visit_oper(0, asa).1),
    };

    match oper {
        Oper::Num(x) => (idx, *x),

        Oper::Pos => visit_oper(idx+1, asa),
        Oper::Neg => {
            let (idx, x) = visit_oper(idx+1, asa);
            (idx, -x)
        },
        
        Oper::Add => {
            let (idx, x) = visit_oper(idx+1, asa);
            let (idx, y) = visit_oper(idx+1, asa);

            (idx, x + y)
        },
        
        Oper::Sub => {
            let (idx, x) = visit_oper(idx+1, asa);
            let (idx, y) = visit_oper(idx+1, asa);

            (idx, x - y)
        },
        
        Oper::Mul => {
            let (idx, x) = visit_oper(idx+1, asa);
            let (idx, y) = visit_oper(idx+1, asa);

            (idx, x * y)
        },
        
        Oper::Div => {
            let (idx, x) = visit_oper(idx+1, asa);
            let (idx, y) = visit_oper(idx+1, asa);

            (idx, x / y)
        },
    }
}

fn parse_paren(tokens: &mut SpannedIter<'static, Token>, parent: Option<usize>) -> Result<(NodeInfo, Vec<Node<Oper>>, bool), Vec<KError<Token, Error>>> {
    let start = tokens.span();
    let parsed = Parser::new(tokens, Some(Token::RParen), token_informer).parse()?; // parse what's inside
    Ok((NodeInfo { // parenthesis information
        span: start.start..tokens.span().end,
        parent,
        precedence: 0,
        space: false,
    }, parsed, false))
}

fn token_informer(token: Token, span: Span, double_space: bool) -> TokInfoOrCustom<Oper, Token, SpannedIter<'static, Token>, Error> {
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

        (T::RParen, _) => return TokInfoOrCustom::Custom(Box::new(move |_, _| Err(vec![KError::Other(span, Error::UnexpectedCharacter)]))),
        (T::LParen, _) => return TokInfoOrCustom::Custom(Box::new(parse_paren)),
    };

    TokInfoOrCustom::TokenInfo(TokenInfo {
        oper,
        span,
        space,
        precedence,
    })
}

fn main() {
    const SRC: &'static str = "-1 + 2 * -3 - 4 / +8 + 272 / (-(1 + 3) * 16)";
    
    let mut lexer = Token::lexer(SRC).spanned();
    let parser = Parser::<Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut lexer, None, token_informer);

    let asa = match parser.parse() {
        Ok(x) => x,
        Err(e) => {
            e.iter().for_each(|x| println!("{x:?}"));
            panic!("an error occurred");
        },
    };

    println!("{asa:?}");

    if asa.is_empty() { return };

    let out = visit_oper(0, &asa);
    println!("result: {out:?}");
}
