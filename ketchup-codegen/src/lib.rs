// The `quote!` macro requires deep recursion.
#![recursion_limit = "196"]

mod parser;
mod errors;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;
use crate::parser::Parser;

const KETCHUP_ATTR: &str = "ketchup";
const KETCHUP_OPER_ATTR: &str = "koper";
const KETCHUP_CUSTOM_ATTR: &str = "kustom";
const KETCHUP_VALUE_ATTR: &str = "kval";

macro_rules! unwrap_compile_error {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => {
                let err = err.into_compile_error();
                return quote::quote! {
                    #err
                };
            },
        }
    }
}

/// Generates the `Parser` implementation
/// 
/// This is *far* less verbose than implementing the `ketchup::Parser` manually
pub fn generate(input: TokenStream) -> TokenStream {
    // let item: ItemEnum = syn::parse2(input).expect("Ketchup can only be derived for enums");
    let item: ItemEnum = unwrap_compile_error!(syn::parse2(input));
    let name = &item.ident;

    quote::quote! {
        impl #name {
            pub fn test() {
                println!("everything is okay!");
            }
        }
    }
}
