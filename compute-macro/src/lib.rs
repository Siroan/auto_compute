extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Formulate)]
pub fn formulate_derive(input: TokenStream) -> TokenStream {
    impl_formulate(&syn::parse(input).unwrap())
}

fn impl_formulate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn formulate() -> String {
                format!("Hello, Macro! My name is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}