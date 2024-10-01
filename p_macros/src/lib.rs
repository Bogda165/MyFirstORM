extern crate proc_macro;
use proc_macro::{TokenStream};
use std::any::{type_name, type_name_of_val};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{parse, parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, ItemFn, ItemStruct, LitStr, Type};
use syn::token::Token;
use Db_shit::*;

#[derive(Clone)]
struct MetaData <'a>{
    attr_type: HashMap<&'a str, &'a str>,
}
impl<'a> MetaData<'a> {
    pub fn default() -> MetaData<'a> {
        let mut set = HashMap::new();
        set.insert("INTEGER",  "INTEGER");
        set.insert( "FLOAT",  "REAL");
        set.insert( "TEXT",  "TEXT");
        set.insert( "PK",  "PRIMARY KEY");
        set.insert( "AUTO_I",  "AUTOINCREMENT");
        set.insert( "CONNECT", "  ");
        MetaData {
            attr_type: set
        }
    }
}

#[proc_macro_derive(MyTrait2)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();

    impl_macro(&input)
}

fn impl_macro(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let _res = quote! {
        impl MyTrait2 for #name {
            fn print_hello_with_macro(&self) {
                println!("Hi my name is: {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(_res)
}

#[proc_macro_attribute]
pub fn log_exec_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;

    TokenStream::from(quote! {
        use std::time;
        fn #fn_name() {
            let start = time::Instant::now();
            (|| #block)();
            println!("Execition time: {:?} ms", start.elapsed().as_millis());
        }
    })
}

//allow attributes or types

fn is_in_allowed_attrs(input: &Ident) -> Result<((proc_macro2::TokenStream)), (())>
{
    let input_name = input.to_string();
    let attrs = Attributes::get_variants();
    let types = DbTypes::get_variants();

    if attrs.contains(&input_name) {
        Ok(quote! {
            Attributes
        })
    }else if types.contains(&input_name) {
        Ok(quote!{
            DbTypes
        })
    } else {
        return Err(())
    }

}

fn create_attr_with_type(input: &Attribute, field_name: Ident) -> Result<((proc_macro2::TokenStream)), (())> {
    let input_s = input.meta.path().get_ident().unwrap().to_string();
    let attrs = Attributes::get_variants();
    let types = DbTypes::get_variants();
    let input_ident = input.meta.path().get_ident().unwrap();

    if attrs.contains(&input_s) {
        Ok(match input.meta.require_list(){
            Ok(list) => {
                quote!{
                    Attributes::#list
                }
            }
            Err(_) => {
                quote!{
                    Attributes::#input_ident
                }
            }
        })
    }else if types.contains(&input_s) {
        Ok(quote!{
            DbTypes::#input_ident(self.#field_name.clone())
        })
    } else {
        return Err(())
    }
}

fn to_string<'a, 'b>(attr: &'a Ident, meta_data: MetaData<'b>) -> &'b str {
    let attr_name = &*attr.to_string();

    meta_data.attr_type.get(attr_name).unwrap()
}

#[proc_macro_attribute]
pub fn table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!("Start");
    let meta_data = MetaData::default();

    let table_name = parse_macro_input!(_attr as LitStr);
    let table_name_ident = Ident::new(&table_name.value(), table_name.span());
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    let table_struct;
    let impl_table;
    let impl_table_shadow;

    let data = match input.data {
        Data::Struct(data) => {
            data
        }
        _ => {
            panic!("Not a structure");
        }
    };
    let _ = {
        let mut construct_table_s = HashMap::<Ident, Vec<Attribute>>::new();
        for field in data.fields.iter() {
            construct_table_s.insert(field.clone().ident.unwrap(), field.clone().attrs);
        }

        //create a struct
        table_struct = {
            let construct_table_s = construct_table_s.clone();

            let fields = construct_table_s.iter().map(|field| {
                let name = field.0;
                let attrs = field.1.iter().map(|attr| {
                    match is_in_allowed_attrs(&attr.meta.path().get_ident().unwrap()) {
                        Ok(attr) => {attr}
                        Err(_) => {
                            panic!("Not allowed type or attr")
                        }
                    }
                });

                quote! {
                    pub #name: (#(#attrs),* ),
                }
            });

            quote!{
                pub struct #table_name_ident {
                    #(#fields)*
                }
            }
        };
        // create an impl
        impl_table = {
            let fields = construct_table_s.iter().map(|field|{
                let name = field.0;
                let attrs = field.1.iter().map(|attr| {
                    match create_attr_with_type(attr, name.clone()) {
                        Ok(attr) => {attr}
                        Err(_) => {
                            panic!("Not allowed type or attr")
                        }
                    }
                });
                quote! {
                    #name: (#(#attrs),* ),
                }
            });
            quote!(
                #table_name_ident
                {
                    #(#fields)*
                }
            )
        };
        //create an impl for table
        impl_table_shadow = {
            let create = {
                let mut query = String::from(format!("CREATE TABLE {} (\n", table_name_ident.to_string()));
                construct_table_s.iter().for_each(|field| {
                    query += &field.0.to_string();
                    query += "\t";
                    field.1.iter().for_each(|attr| {
                        query += to_string(attr.meta.path().get_ident().unwrap(), meta_data.clone());
                        query += " "
                    });
                    query += ",\n"
                });
                query.pop();
                query.pop();
                query += "\n);";
                query
            };

            let mut size = 0;

            let tuple = {
                let fields_vals = construct_table_s.iter().map(|field| {
                    size += 1;
                    let name = field.0;
                    let mut res = quote!{#name};
                    if field.1.len() != 1 {
                        res = quote! {#name.0};
                    }

                    quote! {
                        #res
                    }
                });

                quote! {
                     (#((self.#fields_vals).for_insert(10)), *)
                }
                //let mut query = String::from(format!("INSERT INTO {} ({}) VALUES \n", table_name_ident.to_string(), fields_names));
            };

            let paren = {

                let strings = (0..size).into_iter().map(|val| {
                    quote! {
                            String
                        }
                });

                quote! {
                    (#(#strings), *)
                }

            };

            quote! {
                impl #table_name_ident{
                    pub fn create(&self) -> String{
                        #create.to_string()
                    }

                    pub fn insert(&self) -> #paren {
                        //INSERT INTO .... VALUES
                        //(self.id.for_insert(), self.text.for_insert())
                        #tuple
                    }
                }
            }
        };
    };

    let fields = match data.fields {
        Fields::Named(fields) => {
            let not_t_fields = fields.named.into_iter().map(|mut field| {
                field.attrs.clear();
                field
            });

            quote! {  #(pub #not_t_fields),* }
        }
        _ => {
            panic!("fields are not named wtf")
        }
    };

    //connect
    TokenStream::from(quote!{
        pub mod #table_name_ident{
            use Db_shit::*;
            #[derive(Debug)]
            #table_struct
            #impl_table_shadow

            pub struct #name
            {
                #fields
            }

            impl Entity for #name
            {
                fn get_table_name() -> String {
                    #table_name.to_string()
                }

            }

            impl #name {
                pub fn get_table2(&self) -> #table_name_ident {
                    #impl_table
                }
            }
        }
    })
}

/*
Common Attributes:

1.	PRIMARY KEY: Uniquely identifies a row.
2.	AUTOINCREMENT: Automatically increments integer values.
3.	NOT NULL: Ensures no null values.
4.	UNIQUE: Enforces uniqueness.
5.	CHECK: Applies custom constraints.
6.	DEFAULT: Specifies default values.
7.	COLLATE: Defines string collation rules.
8.	FOREIGN KEY: Establishes relationships between tables.
9.	ON CONFLICT: Specifies conflict-handling behavior.

*/