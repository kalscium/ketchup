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

pub fn generate(input: TokenStream) -> TokenStream {
    let item: ItemEnum = syn::parse2(input).expect("Ketchup can only be derived for enums");

    let name = &item.ident;

    todo!()
}
