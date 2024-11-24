//! Interprets the ASA

use crate::{parser::Expr, span::Spanned};

pub fn walk(asa: &mut impl Iterator<Item = Spanned<Expr>>) -> f64 {
    let next = asa.next().unwrap(); // alright cuz ASA must be valid
    match next.item {
        // operands
        Expr::Number(num) => num,
        Expr::Scoped(expr) => walk(&mut expr.item.vector.into_iter()),

        // unary
        Expr::Pos => walk(asa),
        Expr::Neg => -walk(asa),

        // binary
        Expr::Add => walk(asa) + walk(asa),
        Expr::Sub => walk(asa) - walk(asa),
        Expr::Mul => walk(asa) * walk(asa),
        Expr::Div => walk(asa) / walk(asa),
    }
}
