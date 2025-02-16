use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, Fields};
use crate::additional_functions::docs_manipulations::from_attribute_to_comment;

pub fn update_fields(data: DataStruct, name: &Ident) -> proc_macro2::TokenStream {
    let mut comments_q = Default::default();
    let fields = match data.fields {
        Fields::Named(fields) => {
            let not_t_fields = fields.named.into_iter().map(|mut field| {
                comments_q = {
                    let comments = field.attrs.iter().map(|attribute| {
                        //eprintln!("{:?}", attribute.meta.path().get_ident().unwrap().to_string());
                        from_attribute_to_comment(attribute.clone())
                    });

                    quote! {
                        #(#comments)*
                    }
                };

                {
                    field.attrs.clear();
                }

                return quote! {
                    #comments_q
                    pub #field,
                }

            });

            quote! {
                #(#not_t_fields)*
            }
        }
        _ => {
            std::panic!("fields are not named wtf")
        }
    };
    quote! {
        #[dsl::table]
        pub struct #name {
            #fields
        }
    }
}