use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DataStruct, DeriveInput, WhereClause};
use syn::StrStyle::Cooked;
use crate::additional_functions::functions::{get_inside_attrs, iter_through_attrs};

fn create_columns_from_row(mut table: DataStruct) -> TokenStream2{
    let res = table.fields.iter_mut().filter_map(|field| {
        let temp_vec = iter_through_attrs(field, false,  |field, attr_name, attr| {
            let field_name = field.ident.clone().unwrap();
            let field_name_str = field_name.to_string();
            match &*attr_name {
                "column" => {
                    Some(quote! {
                        #field_name: row.get::<&str, <#field_name as TheType>::Type>(#field_name_str).unwrap().into()
                    })
                }
                _ => {
                    None
                }
            }
        });

        if temp_vec.len() == 0{
            None
        }else {
            Some(temp_vec[0].clone())
        }
    });

    quote!{
        #(#res),*
    }
}

pub(crate) enum ConnectionType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

fn create_add_function(mut table: DataStruct) -> TokenStream2 {
    let inside = table.fields.iter_mut().filter_map(|field| {
        let mut connect_table_name = None;
        let mut connection_type = ConnectionType::OneToOne;
        iter_through_attrs(field, false, |field, attr_name, attr| {
            match &*attr_name {
                "connect" => {
                    let mut attrs = get_inside_attrs(field, &*attr_name, |attr_ident| {
                        Ok(attr_ident)
                    });

                    // if attrs.len() > 1 {
                    //     panic!("connect attribute must be used only for one table at the time")
                    // }

                    assert!(attrs.len() > 0);

                    connect_table_name = if connect_table_name.is_none() { Some(attrs.remove(0)) } else {None};

                    Some(quote!{})
                }
                "connect_type" => {
                    let mut attrs = get_inside_attrs(field, &*attr_name, |attr_ident| {
                        Ok(attr_ident)
                    });

                    assert_eq!(attrs.len(), 1);

                    match &*attrs[0].to_string() {
                        "OneToOne" => {connection_type = ConnectionType::OneToOne},
                        "OneToMany" => {connection_type = ConnectionType::OneToMany},
                        "ManyToMany" => {connection_type = ConnectionType::ManyToMany},

                        _ => {
                            panic!("No such connection type as {}", attrs[0])
                        }
                    }
                    Some(quote!{})
                }
                _ => {
                    None
                }
            }
        });

        let field_name = field.clone().ident.unwrap();

        let connect_table_name = match connect_table_name {
            None => {return None;}
            Some(name) => {name}
        };
        let connect_table_name_string = connect_table_name.to_string();

        let inside_impl = quote! {
            let any_obj = obj.clone_box().into_any();
            let obj = any_obj.downcast::<#connect_table_name>().unwrap();
        };

        let res_q = match connection_type {
            ConnectionType::OneToOne => {

                quote!{
                    assert_eq! {vector.len(), 1}

                    let obj: &Box<dyn DbResponseConv> = vector.remove(0);

                    #inside_impl

                    self.#field_name = *obj
                }
            }
            ConnectionType::OneToMany => {
                quote!{
                    let _vec: Vec<_> = vector.into_iter().map(|obj| {
                        #inside_impl
                        *obj
                    }.collect());
                    self.#field_name.extend(_vec);
                }
            }
            ConnectionType::ManyToMany => {unreachable!()}
        };

        Some(quote! {
            #connect_table_name_string => {
                #res_q
            }
        })
    });

    quote! {
        fn add(&mut self, table_name: TableName, mut vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                #(#inside)*
                _ => {
                    println!("there no such field in entity as {}", table_name.name);
                }
            }
        }
    }
}

fn create_for_every_impl(mut table: DataStruct) -> TokenStream2{

    let res = table.fields.iter_mut().filter_map(|field|{
        let mut connect_table_name = None;
        iter_through_attrs(field, false, |field, attr_name, attr| {
            match &*attr_name {
                "connect" => {
                    let mut attrs = get_inside_attrs(field, &*attr_name, |attr_ident| {
                        Ok(attr_ident)
                    });

                    assert!(attrs.len() > 0);

                    connect_table_name = if connect_table_name.is_none() { Some(attrs.remove(0)) } else {None};

                    Some(quote!{})
                }
                _ => {None}
            }
        });

        let connect_table_name = match connect_table_name {
            None => {
                return None;
            }
            Some(table) => {
                table
            }
        };

        let connect_table_name_string = connect_table_name.to_string();

        Some(quote!{
            #connect_table_name_string => {
                func(Box::new(#connect_table_name::default()), eq)
            }
        })
    });

    quote!{
        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                #(#res)*
                 _ => {
                    println!("there no such field in entity as {}", tb.name);
                }
            }
        }

    }
}

fn get_name_func_func(mut table: DataStruct) -> TokenStream2 {

    let fields = table.fields.into_iter().filter_map( |mut field|{
        let field_name = field.clone().ident.unwrap();
        let field_name_str = field_name.to_string();
        let mut _res = iter_through_attrs(&mut field, false, |field, name, attr| {
            match &*name {
                "column" => {
                    Some(quote!{
                        #field_name_str => {
                            let tmp: <#field_name as TheType>::Type = self.#field_name.clone().into();
                            tmp.into()
                        },
                    })
                }
                _ => {
                    None
                }
            }
        });

        return if _res.is_empty() {
            None
        } else {
            Some(_res.remove(0))
        }
    });

    quote!{
       fn get_by_name(&self, name: &String) -> Value {
            match name.as_str() {
                #(#fields)*
                _ => {
                    unreachable!()
                }
            }
        }
    }
}

pub fn load_funcs(input: DeriveInput) -> TokenStream2 {
    let table_name = &input.ident;
    let generics = &input.generics;
    let where_clause = match generics.where_clause {
        None => {quote! {}}
        Some(ref where_c) => {
            quote!{#where_c}
        }
    };

    let table = if let Data::Struct(table) = &input.data {
        table
    }else {
        panic!("The input must be a structure");
    };

    let columns_from_row = create_columns_from_row(table.clone());
    let add_func = create_add_function(table.clone());
    let for_ever_func = create_for_every_impl(table.clone());
    let get_name_func = get_name_func_func(table.clone());

    quote!{
        impl #generics DbResponseConv for #table_name #generics #where_clause {
            fn into_any(self: Box<Self>) -> Box<dyn Any> {self}
            fn clone_box(&self) -> Box<dyn DbResponseConv> {Box::new(self.clone())}

            fn default_obj(&self) -> Box<dyn DbResponseConv> {Box::new(#table_name::default())}

            fn from_response(&self, row: &Row)  -> Box<dyn DbResponseConv> {
                Box::new(#table_name {
                    #columns_from_row,

                    ..Default::default()
                })
            }

            #add_func
            #for_ever_func
            #get_name_func
        }


    }
}