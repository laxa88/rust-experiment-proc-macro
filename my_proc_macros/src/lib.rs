use proc_macro::TokenStream; // no need to import a specific crate for TokenStream
use syn::parse;

// #[proc_macro]
// pub fn my_function_proc_macro(input: TokenStream) -> TokenStream {
//     input
// }

#[proc_macro_derive(AddMyField)]
pub fn my_derive_proc_macro(input: TokenStream) -> TokenStream {
    input
}

// #[proc_macro_attribute]
// pub fn my_attribute_proc_macro(args: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
