use ariadne::sources;
use logos::Logos;
use maths_interpreter::{error, parser, token::Token};

fn main() {
    let example = r##"
        // 1 + 2 * -3 / +(4 - 5) *+ (12)-
        1 * 2 +
    "##;
    let filename = "foo.bar";

    let mut tokens = Token::lexer(&example).spanned();

    let expr = match parser::parse(&mut tokens, filename) {
        Ok(expr) => expr,
        Err(err) => {
            error::print(err, sources(vec![(filename.to_string(), example)]));
            panic!("an error occured");
        },
    };

    println!("parsed expr: {expr:?}");
}
