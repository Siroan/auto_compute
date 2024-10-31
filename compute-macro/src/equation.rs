use proc_macro::TokenStream;
use syn::{Data, Fields};

use crate::field::Field;
use crate::symbols::VARIABLE;

fn log_structure(message: String) {
    logger::log(logger::LogStep::Structure, &message);
}

pub fn expand_derive_equation(input: &mut syn::DeriveInput) -> TokenStream {
    let mut variables = vec![];

    log_structure(format!("Parsing equation {:?}", input.ident.to_string()));

    let ident = &input.ident;
    let mut struct_diagnostics = quote! {};
    if let Data::Struct(data) = input.clone().data {
        if let Fields::Named(fields) = data.fields {
            fields.named.iter().for_each(|field| {
                for attr in &field.attrs {
                    if attr.path() == VARIABLE {
                        if let Some(ident) = field.clone().ident {
                            log_structure(format!("Found variable: {:?}", ident.to_string()));
                            variables.push(Field { name: ident });
                        }
                    } else {
                        break;
                    }
                }
            });
        } else {
            struct_diagnostics = quote! {
                #struct_diagnostics
                compile_error!("The equation must contain named fields");
            }
        }
    } else {
        struct_diagnostics = quote! {
            #struct_diagnostics
            compile_error!("The equation must be a struct");
        }
    }

    let mut find_unknown = quote! {};
    for variable in variables {
        let name = variable.name;
        let name_s = name.to_string();
        find_unknown = quote! {
            #find_unknown

            log_setup(format!("Variable \"{}\" is {:?}", #name_s, self.#name));

            if self.#name.is_unknown() {
                if unknown.is_some() {
                    log_setup(format!("Error: Several unknown"));
                    return Err(Error::SeveralUnknown);
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
                use compute::error::Error;

                #struct_diagnostics

                fn log_setup(message: String) {
                    logger::log(logger::LogStep::Setup, &message);
                }

                let mut unknown = None;
                #find_unknown

                match unknown {
                    None => {
                        log_setup(format!("Error: No unknown"));
                        Err(Error::NoUnkown)
                    },
                    Some(unknown) => {
                        self.auto_compute();
                        unknown.get_unknown_value()
                    }
                }
            }
        }
    }
    .into()
}
