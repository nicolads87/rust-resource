extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;


extern crate reqwest;

#[proc_macro_derive(RResource)]
pub fn rresource_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_rresource_macro(&ast)
}


fn impl_rresource_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl RResource for #name {
            fn test(&self) {
                println!("Hello, Test! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}