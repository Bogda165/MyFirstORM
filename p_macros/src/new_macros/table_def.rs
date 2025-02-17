use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Field, Path, PathSegment, Token, Type};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;

fn get_last_ident(path: &Path) -> Option<&Ident> {
    path.segments.last().map(|segment: &PathSegment| &segment.ident)
}

fn compare_path(path1: &Path, path2: &Path) -> bool{
    path1.segments.iter()
        .map(|seg| &seg.ident)
        .cmp(
            path2.segments.iter()
                .map(|seg| &seg.ident)
        ).is_eq()
}


pub fn impl_from(types: Punctuated<Type, Token![,]>) -> TokenStream2 {
    // Parse the input as a Punctuated list of Type, separated by commas

    let expanded = types.clone().into_iter().map(|_type| {
        match _type {
            Type::Path(tp) => {
                let _type = tp.path;
                //let type_ident = get_last_ident(&_type).unwrap();
                let impls_for_other_tables = types.clone().into_iter().map(|new_type| {
                    match new_type {
                        Type::Path(tp) => {
                            let new_type = tp.path;
                            //let new_type_ident = get_last_ident(&new_type).unwrap();

                            if compare_path(&_type, &new_type) {return quote! {}}

                            quote! {
                                 impl <U: Allowed<#_type>> Allowed<#_type> for (#new_type, U){}
                            }
                        }
                        _ => panic!("Incorrect type")
                    }
                });

                quote! {
                    impl<T> Allowed<#_type> for (#_type, T){}

                    #(#impls_for_other_tables)*
                }
            }
            _ => quote!{}
        }
    });

    quote! {
        #(#expanded)*
    }
}

/// if the F func the None if the attribute wasn't find, and Some(index, TokenStream) if was
pub fn iter_through_attrs<MatchF>(field: &mut Field, delete_attrs: bool, func: MatchF) -> Vec<TokenStream2>
where
    MatchF: Fn(&Field, String) -> Option<TokenStream2>,
{
    let attrs_amount = field.attrs.len();
    let mut remove_attrs: Vec<usize> = vec![];

    let opened_attrs= field.attrs.iter().zip(0..attrs_amount).map(|(attr, index)| {
        match attr.meta.path().get_ident() {
            None => { quote! {} }
            Some(attr) => {
                if let Some(ts)  = func(field, attr.to_string()) {
                    remove_attrs.push(index);
                    ts
                }else {
                    quote!{}
                }
            }
        }
    });

    let _result = opened_attrs.collect();

    if delete_attrs {
        eprintln!("Removing attrs: {:?}", remove_attrs);
        let length = remove_attrs.len();
        remove_attrs.into_iter().zip(0..length).for_each(|(index, remove_i)| {
            field.attrs.remove(remove_i - index);
        });
    }

    _result
}

pub fn impl_table(mut table: DeriveInput, delete_attrs: bool, table_attrs: TokenStream2) -> TokenStream2{

    let table_name = table.clone().ident;

    let table_name_string = table_name.to_string();

    let _impl = match table.data {
        Data::Struct(ref mut table) => {
            table.fields.iter_mut().map(|field| {

                let field_name = field.clone().ident.unwrap();
                let field_type = field.clone().ty;
                let field_name_string = field_name.clone().to_string();

                let opened_attrs = iter_through_attrs(field, delete_attrs,
                        |field, attrs_name|{
                                match &*attrs_name {
                                    "column" =>  {

                                        Some(quote! {
                                            pub struct #field_name;

                                            impl Default for #field_name {
                                                fn default() -> Self {
                                                    #field_name {}
                                                }
                                            }

                                            impl TheType for #field_name {
                                                type Type = #field_type;
                                            }


                                            impl Into<RawTypes> for #field_name {
                                                fn into(self) -> RawTypes {
                                                    RawTypes::Column(RawColumn{ table_name: #table_name_string.to_string(), name: #field_name_string.to_string() })
                                                }
                                            }

                                            impl Column for #field_name {
                                                type Table = #table_name;

                                                fn get_name() -> String {
                                                    #field_name_string.to_string()
                                                }
                                            }
                                        })
                                    }
                                    "null" => {
                                        Some(quote! {
                                            impl ConvertibleTo<Null> for #field_name {}
                                        })
                                    }
                                    _ => {
                                        None
                                    }
                                }
                        });

                quote! {
                    #(#opened_attrs)*
                }
            })
        }
        _ => {
            panic!("must be a struct")
        }
    };



    quote! {
        use dsl::column::Column;
        use dsl::column::RawColumn;
        use dsl::expressions::raw_types::RawTypes;
        use dsl::convertible::TheType;
        use dsl::column::Allowed;
        use dsl::column::Table;

        impl Table for #table_name {
            fn get_name() -> String {
                #table_name_string.to_string()
            }
        }
        impl Allowed<#table_name> for #table_name {}


        #(#_impl) *
        #table_attrs
        pub #table
    }
}