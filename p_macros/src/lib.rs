
extern crate proc_macro;
use proc_macro::{TokenStream};
use std::default::Default;
use std::panic;
use std::str::FromStr;
use std::vec::IntoIter;
use proc_macro2::{Ident, Literal, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{parenthesized, parse, parse_macro_input, parse_quote, token, AttrStyle, Attribute, Data, DataStruct, DeriveInput, Error, Expr, ExprLit, Field, Fields, FieldsNamed, Item, ItemFn, ItemStruct, LitStr, MacroDelimiter, Meta, MetaList, Path, Token, Type};
use syn::__private::TokenStream2;
use syn::Lit::Str;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::token::{Struct, Token};
use Db_shit::*;
use macros_helper_function::additional_functions::construct_table::create_construct_table_from_doc;
use macros_helper_function::additional_functions::docs_manipulations::from_attribute_to_comment;
use macros_helper_function::additional_functions::functions::{iter_through_attrs, orm_table_derive_f};
use macros_helper_function::meta_data::MetaData;
use macros_helper_function::modify_basic_struct::main_function::create_macro;
use macros_helper_function::new_macros::table_def::impl_from;
use macros_helper_function::repo_struct::main_function::init_repo_struct;

macro_rules! my_to_vec {
    ($vec:expr, Vec<$vec_type:ty>, $left:literal, $right:literal) => {
        {
            let _vec: [$vec_type; $right - $left] = $vec[..$right - $left].try_into().unwrap();
            _vec
        }
    }
}



fn find_by_name(path: &str, name: String) -> Result<ItemStruct, ()> {
    //convert to syn::File
    let syn_file = match syn::parse_file(path) {
        Ok(file) => file,
        Err(_) => {panic! ("Couldn't open a file")}
    };

    for item in syn_file.items {
        match item {
            Item::Struct(stru) => {
                if stru.ident.to_string() == name {
                    return Ok(stru.clone());
                }
            }
            _ => {}
        }
    }

    Err(())
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


#[proc_macro_attribute]
pub fn table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!("Start");

    let shadow_table_name = parse_macro_input!(_attr as LitStr);
    let shadow_table_name_i = Ident::new(&shadow_table_name.value(), shadow_table_name.span());
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.clone().ident;



    let macro_res = create_macro(input, shadow_table_name_i, name, shadow_table_name);

    TokenStream::from(macro_res)
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

#[proc_macro_attribute]
pub fn repo(_attr: TokenStream, input: TokenStream) -> TokenStream {

    eprintln!("{_attr}");
    let attrs = parse_macro_input!(_attr as macros_helper_function::custom_parser::KeyValueList);

    let mut table_name = LitStr::new("", Span::call_site());
    let mut entity_name = LitStr::new("", Span::call_site());

    eprintln!("hui");

    //TODO refactor

    for val in attrs {
        if val.key == "entity" {
            entity_name = val.value;
        }
        else if val.key == "table" {
            table_name = val.value;
        }
    }

    eprintln!("{:?} {:?}", table_name.value(), entity_name.value());



    let entity_ident = Ident::new(&*entity_name.value(), Span::call_site());
    let table_ident = Ident::new(&*table_name.value(), Span::call_site());
    let repo = parse_macro_input!(input as DeriveInput);

    let repo_name = repo.ident;
    let repo = match repo.data {
        Data::Struct(table) => {table}
        Data::Enum(_) => {
            panic!("It should be a struct")
        }
        Data::Union(_) => {
            panic!("It should be a struct")
        }
    };

    TokenStream::from(init_repo_struct(&repo, &repo_name, &entity_ident, &table_ident))
}

#[proc_macro]
// This macro must be used in the root of the project to generate multiple tables types
pub fn from(input: TokenStream) -> TokenStream {

    let types = parse_macro_input!(input with Punctuated::<Type, Token![,]>::parse_terminated);

    TokenStream::from(impl_from(types))
}


#[proc_macro_attribute]
pub fn attrs_to_comments(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut table = parse_macro_input!(input as DeriveInput);

    macros_helper_function::additional_functions::functions::attrs_to_comments_f(&mut table);

    TokenStream::from(quote!{
        #table
    })
}


#[proc_macro_derive(OrmTable)]
pub fn orm_table_derive(input: TokenStream) -> TokenStream {
    let mut _table = parse_macro_input!(input as DeriveInput);

    let _final = orm_table_derive_f(_table);

    TokenStream::from(_final)
}


// #[proc_macro_derive(OrmRepo)]
// pub fn orm_repo_derive(input: TokenStream) -> TokenStream {
//     let mut repo_struct = parse_macro_input!(input as DeriveInput);
//
//     let _final = orm_repo_derive_f(repo_struct);
//
//     return TokenStream::from(_final)
//
// }

#[proc_macro_attribute]
pub fn data_base(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut db_struct = parse_macro_input!(input as DeriveInput);

    eprintln!("parsed");
    let mut table_name_query: Option<String> = None;
    let mut from_tables: Vec<Path> = vec![];

    db_struct.attrs.iter_mut().for_each(|attr| {
        match &*attr.meta.path().get_ident().unwrap().to_string() {
            "name" =>  {
                match attr.meta {
                    Meta::Path(_) => {}
                    Meta::List(_) => {}
                    Meta::NameValue(ref value) => {
                        match value.value {
                            Expr::Lit(ref lit) => {
                                match lit.lit {
                                    Str(ref name) => {
                                        table_name_query = Some(name.clone().value())
                                    }
                                    _ => {
                                        panic!("The name must be defined with string");
                                    }
                                }
                                eprintln!("string lit");
                            }
                            _ => {
                                panic!("the name must be  astring literal");
                            }
                        }
                    }
                }
            }
            "from" => {
                match attr.meta {
                    Meta::Path(_) => {}
                    Meta::List(ref listed_values) => {
                        from_tables = syn::parse::<macros_helper_function::custom_parser::CommaPath>(TokenStream::from(listed_values.clone().tokens)).unwrap().into();
                    }
                    Meta::NameValue(_) => {}
                }
            }
            _ => {}
        }
    });

    let data_base_struct = match &mut db_struct.data {
        Data::Struct(res) => {res}
        Data::Enum(_) => {panic!("must be a struct")}
        Data::Union(_) => {panic!("must be a struct")}
    };

    match &mut data_base_struct.fields {
        Fields::Named(fields) => {
            let new_field = TokenStream::from(quote!({connection: Option<Connection>}));
            fields.named.push(parse_macro_input!(new_field as FieldsNamed).named.get(0).unwrap().clone());
        }
        Fields::Unnamed(_) => {panic!("Fields of the db myst be named")}
        Fields::Unit => {panic!("Fields of the db myst be named")}
    };


    let table_name = db_struct.ident.clone();

    let generics = db_struct.generics.clone();

    if table_name_query == None {
        table_name_query = Some(table_name.to_string());
    }

    db_struct.attrs = vec![];

    TokenStream::from(quote!{
        from!(#(#from_tables), *);

        #[derive(Default)]
        #db_struct

        impl OrmDataBase for #table_name{

            fn get_connection(&self) -> &Option<Connection> {
                &self.connection
            }

            fn get_connection_mut(&mut self) -> &mut Option<Connection> {
                &mut self.connection
            }

            fn get_name() -> String {
                #table_name_query.to_string()
            }
        }

        impl #generics #table_name #generics {
            fn connect(&mut self) {
                self.connection = Some(
                    Connection::open_with_flags(
                        #table_name_query,
                        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                    ).unwrap()
                )
            }
        }
    })
}