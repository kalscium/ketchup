//! Functions for parsing tokens

use ketchup::{asa::{VectorASA, ASA}, node::{Node, NodeKind}, parse, Precedence};
use logos::SpannedIter;
use crate::{error::Error, span::{Span, Spanned}, token::{self, NextTok, NextTokWith, Token}};

/// An expression 'node' in the ASA
#[derive(Debug, Clone)]
pub enum Expr {
    // operands
    Number(f64),
    Scoped(Box<Spanned<VectorASA<Spanned<Expr>>>>),

    // unary left-aligned
    Pos,
    Neg,

    // binary
    Add,
    Sub,
    Mul,
    Div,
}

impl Node for Expr {
    fn get_kind(&self) -> NodeKind {
        match self {
            // operands
            Expr::Number(_) => NodeKind::Operand,
            Expr::Scoped(_) => NodeKind::Operand,

            // unary
            Expr::Pos => NodeKind::Unary,
            Expr::Neg => NodeKind::Unary,

            // binary
            Expr::Add => NodeKind::Binary,
            Expr::Sub => NodeKind::Binary,
            Expr::Mul => NodeKind::Binary,
            Expr::Div => NodeKind::Binary,
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            // operands
            Expr::Number(_) => Precedence::MAX,
            Expr::Scoped(_) => Precedence::MAX,

            // unary
            Expr::Pos => Precedence::MAX-1,
            Expr::Neg => Precedence::MAX-1,

            // binary
            Expr::Mul => 1,
            Expr::Div => 1,
            Expr::Add => 0,
            Expr::Sub => 0,
        }
    }
}

/// Parses an iterator of tokens
pub fn parse(
    tokens: &mut SpannedIter<Token>,
    filename: &str,
) -> Result<Spanned<VectorASA<Spanned<Expr>>>, Error> {
    let next_tok = token::next_token(filename, tokens)?;

    // ensure that the program isn't empty
    if next_tok.is_none() {
        return Err(Error::EmptyProgram(Span {
            filename: filename.to_string(),
            range: tokens.span(),
        }));
    }

    // parse an expr
    let NextTokWith { item: expr, next_tok } = parse_expr(next_tok, tokens, filename)?;

    // make sure there are no more tokens following the expression
    if let Some(Spanned { span, .. }) = next_tok {
        return Err(Error::UnexpectedCharacter(span));
    }

    Ok(expr)
}

/// Parses an expr from a iterator of tokens
pub fn parse_expr(
    first_tok: NextTok,
    tokens: &mut SpannedIter<Token>,
    filename: &str
) -> Result<NextTokWith<VectorASA<Spanned<Expr>>>, Error> {
    let mut asa = VectorASA::new();
    let start_span = first_tok.as_ref().map(|Spanned { span, .. }| span.clone());

    // iterate through all the tokens and parse each of them
    let mut current_tok = first_tok;
    while let Some(Spanned { item: token, span }) = current_tok {
        // parse the current token
        match token {
            // operands
            Token::Number(num) => parse::operand(Spanned::new(Expr::Number(num), span), &mut asa)?,
            Token::LParen => parse_paren(span, tokens, &mut asa)?,

            // unary left-aligned nodes (only if the ASA is incomplete)
            Token::Plus if !*asa.is_complete() => parse::unary_left_align(Spanned::new(Expr::Pos, span), &mut asa)?,
            Token::Dash if !*asa.is_complete() => parse::unary_left_align(Spanned::new(Expr::Neg, span), &mut asa)?,

            // binary nodes
            Token::Plus => parse::binary_node(Spanned::new(Expr::Add, span), true, &mut asa)?,
            Token::Dash => parse::binary_node(Spanned::new(Expr::Sub, span), true, &mut asa)?,
            Token::Star => parse::binary_node(Spanned::new(Expr::Mul, span), true, &mut asa)?,
            Token::Slash => parse::binary_node(Spanned::new(Expr::Div, span), true, &mut asa)?,

            // tokens that the parser doesn't recognise
            _ => {
                // in this case, we should make sure the ASA is valid and then return the unknown token alongside the spanned ASA
                parse::ensure_completed(&mut asa)?;
                let expr_span = Span {
                    filename: filename.to_string(),
                    range: start_span.unwrap().range.start..asa.get_node(asa.get_len()-1).span.range.end,
                };

                return Ok(NextTokWith {
                    item: Spanned::new(asa, expr_span.clone()),
                    next_tok: Some(Spanned::new(token, span)),
                })
            },
        }

        // update the current token
        current_tok = token::next_token(filename, tokens)?;
    }

    // ensure that the ASA is valid and completed and then return the expr with no next token
    parse::ensure_completed(&mut asa)?;
    let span = Span {
        filename: filename.to_string(),
        range: start_span.unwrap().range.start..asa.get_node(asa.get_len()-1).span.range.end,
    };
    Ok(NextTokWith {
        item: Spanned::new(asa, span),
        next_tok: None,
    })
}

/// Parses parentheses (encapsulated expressions) (assuming the opening '(' token has been consumed already)
pub fn parse_paren(
    start_span: Span,
    tokens: &mut SpannedIter<'_, Token>,
    asa: &mut impl ASA<Node = Spanned<Expr>>,
) -> Result<(), Error> {
    // check for empty parentheses
    let next_tok = token::next_token(&start_span.filename, tokens)?;
    match next_tok {
        // closing parentheses
        Some(Spanned { item: Token::RParen, span }) =>{
            return Err(Error::EmptyParen {
                span: Span { filename: start_span.filename.clone(), range: start_span.range.start..span.range.end },
                expected_span: Span { filename: start_span.filename, range: start_span.range.end..start_span.range.end },
            });
        },
        // EOF
        None => {
            return Err(Error::EmptyParen {
                expected_span: Span { filename: start_span.filename.clone(), range: start_span.range.end..start_span.range.end },
                span: start_span,
            });
        },
        // expr (okay)
        _ => (),
    }
    
    // parse the internal expr
    let NextTokWith { item: expr, next_tok } = parse_expr(next_tok, tokens, &start_span.filename)?;

    // parse for the closing ')' character
    let end_span = match next_tok {
        // make sure the parentheses are closed
        Some(Spanned { item: Token::RParen, span }) => span,

        // expected closing parenthesis
        None => return Err(Error::UnclosedParen {
            expected_span: Span { filename: start_span.filename.clone(), range: tokens.span() },
            start_span,
        }),

        // expected closing parenthesis instead
        Some(Spanned { span, .. }) => return Err(Error::UnclosedParen {
            start_span,
            expected_span: span,
        }),
    };

    // append the succesfully parsed parentheses (scope)
    parse::operand(
        Spanned::new(
            Expr::Scoped(Box::new(expr)),
            Span {
                filename: start_span.filename,
                range: start_span.range.start..end_span.range.end
            }
        ),
        asa,
    ).map_err(|err| err.into())
}
