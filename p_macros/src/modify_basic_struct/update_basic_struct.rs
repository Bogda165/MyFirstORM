use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_quote, DataStruct, Fields};
use syn::__private::TokenStream2;
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
                    if comments.clone().count() > 0 {
                        quote! {
                            #[column]
                            #(#comments)*
                        }
                    } else {
                        quote!{}
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

    let doc_string = format!("Some string");

    crate::new_macros::table_def::impl_table(parse_quote!(
        struct #name {
            #fields
        }
    ), false, quote!{#[derive(Default, p_macros::OrmTable)]})
}