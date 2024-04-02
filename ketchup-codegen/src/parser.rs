use proc_macro2::{Span, TokenStream};
use syn::{Attribute, Expr, Meta, TypePath};
use crate::{errors::{Error, Errors}, KETCHUP_ATTR};

#[derive(Default)]
pub struct Parser {
    pub in_type: Option<TypePath>,
    pub out_type: Option<TypePath>,
    pub out_conv: Option<Expr>,
    pub ketchup_path: Option<TokenStream>,

    pub errors: Errors,
}

impl Parser {
    #[inline]
    fn pull_meta_list(&mut self, attr: &mut Attribute) -> Option<TokenStream> {
        match attr.meta {
            Meta::List(ref mut list) => {
                let tokens = std::mem::replace(&mut list.tokens, TokenStream::new());
                Some(tokens)
            },
            _ => None,
        } 
    }

    #[inline]
    fn err(&mut self, error: Error, span: Span) -> &mut Errors {
        self.errors.err(error, span)
    }
    
    pub fn try_parse_ketchup(&mut self, attr: &mut Attribute) {
        if !attr.path().is_ident(KETCHUP_ATTR) { return };
        let ketchup = match self.pull_meta_list(attr) {
            Some(ketchup) => ketchup,
            None => {
                return;
            },
        };
    }
}
