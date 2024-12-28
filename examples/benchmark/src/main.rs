use std::time::Instant;
use logos::Logos;
use rand::Rng;

pub mod nom_parser;

fn main() {
    // generate the benchmarking expression
    let expr = gen_expr(4 * 1024 * 1024); // ~4MiB expression
    println!("expr: {expr}\n");

    // test them
    println!("ketchup: {}ms", time_ketchup(&expr));
    println!("nom: {}ms", time_nom(&expr));
}

fn time_nom(src: &str) -> u128 {
    let start = Instant::now();
    let _ = nom_parser::parse(src).unwrap();

    (Instant::now() - start).as_millis()
}

fn time_ketchup(src: &str) -> u128 {
    pub use maths_interpreter::{token::Token, parser};

    let start = Instant::now();

    let mut tokens = Token::lexer(src).spanned();
    parser::parse(&mut tokens, "").unwrap();

    (Instant::now() - start).as_millis()
}

/// Generates an extremely large maths expression
pub fn gen_expr(size: usize) -> String {
    let mut result = String::new();

    // use the logic of ketchup *in reverse*
    let mut complete = false;

    let mut idx = 0;
    while idx < size {
        idx += 1;

        if complete {
            // double space

            // add a random operator
            let random = rand::thread_rng().gen_range(0..4);
            result.push_str(match random {
                0 => " + ",
                1 => " - ",
                2 => " * ",
                3 => " / ",
                _ => unreachable!(),
            });

            // mark incomplete
            complete = false;
        } else {
            // add a random operand/operator
            let random = rand::thread_rng().gen_range(0..2); // change to 4 later

            // if 0 then generate a scoped expression
            if random == 0 {
                // generate a random amount of nodes for the scope (within reason)
                let random = rand::thread_rng().gen_range(0..=size-idx);
                // add it to this scope's idx to prevent infinite runtime
                idx += random;

                // generate and push the scoped expr
                let expr = gen_expr(random+1);
                result.push('(');
                result.push_str(&expr);
                result.push(')');

                // mark complete
                complete = true;

                continue;
            }

            match random {
                1 => {
                    // push random number from 1-255 & mark complete
                    result.push_str(&rand::thread_rng().gen_range(0..256).to_string());
                    complete = true;
                },
                2 => result.push('-'), // negative & leave be
                3 => result.push('+'), // positive & leave be
                _ => unreachable!(),
            }
        }
    }

    // if still incomplete then just cap off with a zero
    if !complete {
        result.push('0');
    }

    result
}
