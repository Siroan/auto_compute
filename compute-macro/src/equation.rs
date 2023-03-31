use proc_macro::TokenStream;
use syn::{Data, Fields};

use crate::symbols::UNKNOWN;
use crate::field::Field;

pub fn expand_derive_equation(
    input: &mut syn::DeriveInput,
) -> TokenStream {//Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;
    let mut l = vec![];
    if let Data::Struct(data) = input.clone().data {
        if let Fields::Named(fields) = data.fields {
            fields.named.iter().enumerate().map(|(i, field)|
            for attr in &field.attrs {
                if attr.path() != UNKNOWN {
                    continue;
                }

                //TODO: add check on type

                if let Some(ident) = field.clone().ident {
                    l.push(Field {
                        name: ident,
                    });
                }
            }
        )
            .collect()
        } else {
            //error
        }
    } else {
        //error
    }

    let mut check_unknowns = quote! {};
    for l in l {
        let name = l.name;
        let name_s = name.to_string();
        check_unknowns = quote! {
            #check_unknowns
            println!("{:?}", #name_s);
            println!("{:?}", self.#name);
            if self.#name == EquationElement::Unknown {
                number_of_unknown += 1;
            }
        }
    }

    quote! {
        #[automatically_derived]
        impl #ident {
            fn compute(&self) -> Result<f64, String> {
                use compute::equation::EquationElement;

                let mut number_of_unknown: u8 = 0;
                #check_unknowns
                println!("Number of unknowns: {:?}", number_of_unknown);

                self.auto_compute();
                Ok(0.)
            }
        }
    }.into()
}
