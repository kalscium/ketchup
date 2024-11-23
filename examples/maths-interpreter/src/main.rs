use logos::Logos;
use maths_interpreter::{parser, token::{self, Token}};

fn main() {
    let example = r##"
        1 + 2 * -3 / +(4 - 5)
    "##;
    let filename = "foo.bar";

    let mut tokens = Token::lexer(&example).spanned();
    let expr = parser::parse_expr(token::next_token(filename, &mut tokens).unwrap(), &mut tokens, filename).unwrap();

    println!("parsed expr: {expr:?}");
}
