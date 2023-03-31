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

    let mut find_unknown = quote! {};
    for l in l {
        let name = l.name;
        let name_s = name.to_string();
        find_unknown = quote! {
            #find_unknown
            println!("{:?}", #name_s);
            println!("{:?}", self.#name);
            if self.#name == EquationElement::Unknown {
                if unknown.is_some() {
                    return Err(Error::MoreThanOneUnknown);
                }
                unknown = Some(self.#name.clone());
            }
        }
    }

    quote! {
        use compute::error::Error;

        #[automatically_derived]
        impl #ident {
            fn compute(&self) -> Result<f64, Error> {
                use compute::equation::EquationElement;

                let mut unknown = None;
                #find_unknown
                if unknown.is_none() {
                    return Err(Error::NoUnkown);
                }

                self.auto_compute();
                Ok(0.)
            }
        }
    }.into()
}
