use proc_macro::TokenStream;
use syn::{Data, Fields};

use crate::field::Field;
use crate::symbols::VARIABLE;

pub fn expand_derive_equation(input: &mut syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let mut variables = vec![];
    if let Data::Struct(data) = input.clone().data {
        if let Fields::Named(fields) = data.fields {
            fields.named.iter().for_each(|field| {
                for attr in &field.attrs {
                    if attr.path() == VARIABLE {
                        //TODO: add check on type

                        if let Some(ident) = field.clone().ident {
                            variables.push(Field { name: ident });
                        }
                    } else {
                        break;
                    }
                }
            });
        } else {
            //TODO: compilation error
        }
    } else {
        //TODO: compilation error
    }

    let mut find_unknown = quote! {};
    for variable in variables {
        let name = variable.name;
        //let name_s = name.to_string(); TODO remove?
        find_unknown = quote! {
            #find_unknown
            //println!("{:?}", #name_s);
            log_compilation(format!("{:?}", self.#name));
            match self.#name {
                EquationElement::Unknown(_) => {
                    if unknown.is_some() {
                        return Err(Error::SeveralUnknown);
                    }
                    unknown = Some(self.#name.clone());
                },
                _ => {},
            }
        }
    }

    quote! {
        use compute::error::Error;

        #[automatically_derived]
        impl #ident {
            fn compute(&self) -> Result<f64, Error> {
                use compute::equation::EquationElement;

                fn log_compilation(message: String) {
                    logger::log(logger::LogStep::Compilation, &message);
                }

                let mut unknown = None;
                #find_unknown
                if unknown.is_none() {
                    return Err(Error::NoUnkown);
                }

                self.auto_compute();

                match unknown {
                    Some(EquationElement::Unknown(unknown)) => {
                        unknown.status.and(Ok(*unknown.unknown.borrow()))
                    },
                    _ => { Ok(0.) },
                }
            }
        }
    }
    .into()
}
