# ketchup
---
> A parser that can *ketch - up* with your programming language.

## Example
---
*for a full implementation/example check the `examples` directory*
```rust
use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    CustomError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub enum Oper {
    Num(u32),
    Add,
    Sub,
    Mul,
    Div,
}

fn oper_generator(token: Token, tokens: &mut impl Iterator<Item = (Result<Token, Error>, Span)>, double_space: bool) -> Result<OperInfo<Oper>, Vec<KError<Token, Error>>> {
    use Token as T;
    use Oper as O;

    // precedence determines the order of operations, lower the precedence the 'smaller' it is
    // space determines how much many input nodes it takes, eg `Space::None` is `x`, `Space::Single` is `x input`, `Space::Double` is `input1 x input2`
    // oper is just the kind of operation it is, like a number, addition, etc
    let (precedence, space, oper) = match (token, double_space) {
        (T::Number(x), _) => (0, Space::None, O::Num(x)),
        (T::Plus, _) => (3, Space::Double, O::Add), // larger precedence changes the order of operations
        (T::Minus, _) => (3, Space::Double, O::Sub),
        (T::Star, _) => (2, Space::Double, O::Mul),
        (T::Slash, _) => (2, Space::Double, O::Div),
    };

    Ok(OperInfo {
        oper,
        span: 0..0, // placeholder for logos `.span()`
        space,
        precedence,
    })
}

fn main() {
    // source to parse
    let mut src = [(Ok(Token::Number(1)), 0..1)].into_iter();

    // initialise parser
    let parser = Parser::<'_, Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut src, None, oper_generator);

    // parse and handle errors
    let asa = parser.parse().unwrap();

    // print abstract syntax array
    println!("{asa:?}");
}
```
