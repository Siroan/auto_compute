extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod equation;
mod symbols;
mod field;

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

#[proc_macro_derive(Equation, attributes(unknown))]
pub fn derive_equation(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    equation::expand_derive_equation(&mut input)
        //.unwrap_or_else(to_compile_errors)
        .into()
}

/*fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}*/
