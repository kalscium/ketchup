use std::fmt;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::{quote_spanned, ToTokens, TokenStreamExt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Errors {
    collected: Vec<SpannedError>,
}

impl Errors {
    pub fn err(&mut self, error: Error, span: Span) -> &mut Self {
        self.collected.push(SpannedError {
            error,
            span,
        });

        self
    }

    pub fn render(self) -> Option<TokenStream> {
        let errors = self.collected;

        match errors.len() {
            0 => None,
            _ => Some(quote! {
                fn _logos_derive_compile_errors() {
                    #(#errors)*
                }
            }),
        }
    }
}

pub enum Error {
    InvalidKetchupAttr,
}

#[derive(Debug)]
pub struct SpannedError {
    error: Error,
    span: Span,
}

impl Error {
    #[inline]
    pub fn span(self, span: Span) -> SpannedError {
        SpannedError {
            error: self,
            span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error as E;
        write!(f, "{}", match self {
            E::InvalidKetchupAttr => "Expected `#[ketchup(...)]`".to_string(),
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl ToTokens for SpannedError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let message = self.error.to_string();

        tokens.append_all(quote_spanned!(self.span => {
            compile_error!(#message)
        }))
    }
}
