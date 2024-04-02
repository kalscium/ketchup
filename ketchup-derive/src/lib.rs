use ketchup_codegen::generate;

#[proc_macro_derive(Ketchup)]
pub fn parser(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate(item.into()).into()
}
