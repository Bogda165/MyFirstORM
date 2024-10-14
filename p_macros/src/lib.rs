extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use std::any::{type_name, type_name_of_val};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::panic;
use std::str::FromStr;
use proc_macro2::{Ident, Literal, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{parse, parse_macro_input, parse_quote, AttrStyle, Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, Item, ItemFn, ItemStruct, LitStr, MacroDelimiter, Meta, MetaList, Type};
use syn::Expr::Lit;
use syn::Lit::Str;
use syn::MacroDelimiter::Paren;
use syn::token::{Struct, Token};
use Db_shit::*;

macro_rules! my_to_vec {
    ($vec:expr, Vec<$vec_type:ty>, $left:literal, $right:literal) => {
        {
            let _vec: [$vec_type; $right - $left] = $vec[..$right - $left].try_into().unwrap();
            _vec
        }
    }
}

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
        set.insert("INTEGER_N", "INTEGER");
        set.insert("FLOAT_N", "FLOAT");
        set.insert("TEXT_N", "TEXT");
        set.insert( "CONNECT", "");
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

    match meta_data.attr_type.get(attr_name) {
        None => {panic!("No type or attribute in the list, {}", attr_name)}
        Some(val) => {
            val
        }
    }
}

struct FroConnect {
    path: String,
    table_name: String,
    field_name: String,
}

impl FroConnect {
    fn default() -> Self {
        FroConnect {
            path: "".to_string(),
            table_name: "".to_string(),
            field_name: "".to_string(),
        }
    }
}

fn handle_field_table_struct(field: (&Ident, &Vec<Attribute>)) -> proc_macro2::TokenStream {
    let name = field.0;
    //check if connect logic
    if field.1.len() == 0 {
        return quote!{};
    }
    if field.1[0].meta.path().get_ident().unwrap().to_string() == "Connect" {

        let mut connect_o = FroConnect::default();
        field.1.iter().for_each(|attr| {
            match attr.meta {
                Meta::Path(_) => {
                    panic!("Not expected attribute {}", attr.meta.path().get_ident().unwrap().to_string())
                }
                Meta::List(_) => {
                    panic!("Not expected attribute {}", attr.meta.path().get_ident().unwrap().to_string())
                }
                Meta::NameValue(ref attr) => {
                    match &*attr.path.get_ident().unwrap().to_string() {
                        "path" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.path = lit_str.value()
                                }
                            }
                        }
                        "table_name" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.table_name = lit_str.value()
                                }
                            }
                        }
                        "field_name" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.field_name = lit_str.value()
                                }
                            }
                        }
                        _ => {
                            panic!("Not expected attribute name")
                        }
                    }
                }
            }
        });


        /*
        //find struct with name table_name, and get the type of field_name
        //form quote
        let mut field_type;
        //find struct
        match find_by_name(&*connect_o.path, connect_o.table_name.clone()) {
            Ok(table) => {
                assert_eq!(table.ident.to_string(), connect_o.table_name);
                if let Fields::Named(fields) = table.fields {
                    for field in fields.named {
                        if field.ident.unwrap().to_string() == connect_o.field_name {
                            field_type = field.ty;
                        }
                    }
                }

            }
            Err(err) => {
                panic!("Couldn't find strcut with name {}; error {:?}", connect_o.table_name, err);
            }
        };

         */

        //get Dbtype from file_type


    }

    let attrs = field.1.iter().map(|attr| {
        match is_in_allowed_attrs(&attr.meta.path().get_ident().unwrap()) {
            Ok(_attr) => {
                _attr
            }
            Err(_) => {
                panic!("Not allowed type or attr")
            }
        }
    });

    quote! {
        pub #name: (#(#attrs),* ),
    }
}

fn from_attribute_to_comment(attr: Attribute) -> proc_macro2::TokenStream {
    let name = attr.meta.path().get_ident().unwrap().to_string();
    let additional = match attr.meta {
        Meta::Path(path) => {
            "".to_string()
        }
        Meta::List(metaList) => {
            let mut token_stream = TokenStream::from(metaList.tokens);
            format!("({})", token_stream.into_iter().map(|tree| {
                match tree {
                    TokenTree::Group(_) => { unreachable!("write converting to a group") }
                    TokenTree::Ident(ident) => { ident.to_string()}
                    TokenTree::Punct(_) => {"".to_string()}
                    TokenTree::Literal(lit) => { lit.to_string() }
                }
            }).filter(|str| str.len() > 0).collect::<Vec<String>>().join(", "))
        }
        Meta::NameValue(_) => {
            unreachable!("Idk what to do if so")
        }
    };
    let _return = format!("{}{}", name, additional);
    quote! {
        #[doc = #_return]
        //I mean its strange
    }
}

fn create_construct_table(structure: &DataStruct) -> HashMap<Ident, Vec<Attribute>>{
    let mut construct_table_s = HashMap::<Ident, Vec<Attribute>>::new();
    for field in structure.fields.iter() {
        construct_table_s.insert(field.clone().ident.unwrap(), field.clone().attrs);
    }
    construct_table_s
}


fn parse_string_to_attr(attr_str: String) -> Result<Attribute, String> {
    // get a type of Meta

    let meta = if let Some(index) = attr_str.find('(') {
        //list
        let ident_str = &(&*attr_str)[..index];
        let len = attr_str.len();
        let ident = Ident::new(ident_str, Span::call_site());
        // get between brackets
        let tokens_iter = (&*attr_str)[(index + 1)..len - 1].split(", ").map(|str| {
            match panic::catch_unwind(|| {Ident::new(str, Span::call_site())}) {
                Ok(ident) => {
                    quote!{
                        #ident
                    }
                }
                Err(_) => {
                    let lit = Literal::from_str(str).unwrap();

                    quote! {
                        #lit
                    }
                }
            }
        });
        let tokens = quote! {
            #(#tokens_iter),*
        };

        eprintln!("{:?}", tokens.to_string());

        let hui = MetaList {
            path: parse_quote!(#ident),
            delimiter: Paren(syn::token::Paren::default()),
            tokens,
        };

        Meta::List(hui)
    }else if let Some(index) = attr_str.find("=") {
        //nameVal
        unreachable!("Not realized for name val")
    }else {
        //path
        let ident_str = &*attr_str;

        let ident = Ident::new(ident_str, Span::call_site());
        Meta::Path(parse_quote!(#ident))
    };



    Ok(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta,
    })
}

//TODO test
fn from_doc_text_to_ident(doc: &Attribute) -> Result<Attribute, String> {
    if doc.meta.path().get_ident().unwrap().to_string() == "doc" {
        let meta_name_value = doc.meta.require_name_value().unwrap();
        if let Lit(value_in_doc) = &meta_name_value.value {
            if let Str(value_in_doc_string) = &value_in_doc.lit {
                let value = value_in_doc_string.value().to_string();
                let attr = parse_string_to_attr(value)?;
                return Ok(attr)
            }
            return Err("Not a String".to_string());
        }
        return Err("Not a Lit".to_string());
    }
    Err("Not a doc".to_string())
}

fn create_construct_table_from_doc(structure: &DataStruct) -> HashMap<Ident, Vec<Attribute>>{
    let mut construct_table_s = HashMap::<Ident, Vec<Attribute>>::new();
    for field in structure.fields.iter() {
        //extract a value from a string
        let _attrs: Vec<Attribute> = field.attrs.iter().map(|attribute| {
            from_doc_text_to_ident(attribute).unwrap()
        }).collect();


        construct_table_s.insert(field.clone().ident.unwrap(), _attrs);
    }
    construct_table_s
}


#[proc_macro_attribute]
pub fn impl_table(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //check for struct
    let input = parse_macro_input!(input as DeriveInput);
    let table_name_ident = &input.ident;
    let table = match &input.data {
        Data::Struct(table) => {table}
        _ => {
            panic!("The table must be represented as a struct")
        }
    };

    let construct_table_s = create_construct_table_from_doc(&table);
    eprintln!("Construct table create from comments");
    let meta_data = MetaData::default();
    //create an impl for table
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
    eprintln!("create method created");
    let mut size: usize = 0;

    let vals = {
        let fields_vals = construct_table_s.iter().map(|field| {
            size += 1;
            let name = field.0;
            let mut res = quote! {#name};
            if field.1.len() != 1 {
                res = quote! {#name.0};
            }

            quote! {
                #res
            }
        });
        quote! {
                 #(&self.#fields_vals), *
            }
        //let mut query = String::from(format!("INSERT INTO {} ({}) VALUES \n", table_name_ident.to_string(), fields_names));
    };
    eprintln!("vals generated");

    let mut question_marks = "".to_string();

    let paren = {
        let _return = (0..size).map(|_| {
            question_marks += "?, ";
            quote! {
                &DbTypes
            }
        });

        quote! {
            (#(#_return), *)
        }
    };
    question_marks.pop();
    question_marks.pop();
    eprintln!("question marks generated");

    let ident = {
        let mut string = "".to_string();

        construct_table_s.iter().for_each(|field| {
            string += &*field.0.to_string();
            string += " ,"
        });
        string.pop();
        string.pop();

        quote! {
            #string
        }
    };
    eprintln!("ident is generated");

    let ts = table_name_ident.to_string();
    eprintln!("{:?}", ident.to_string());

    TokenStream::from(quote! {
        #input
        impl #table_name_ident{
            pub fn create(& self) -> String{
                #create.to_string()
            }
            pub fn insert(&self) -> (String, #paren){
                let query = format!("INSERT INTO {} ({}) VALUES ({});", #ts, #ident.to_string() , #question_marks);
                (query, (#vals))
            }
        }
    })
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

    let data = match input.data {
        Data::Struct(data) => {
            data
        }
        _ => {
            panic!("Not a structure");
        }
    };

    let mut construct_table_s = create_construct_table(&data);

    //create a struct
    table_struct = {
        let construct_table_s = construct_table_s.clone();

        let fields = construct_table_s.iter().map(|field| {
            let _field = handle_field_table_struct(field);

            let attributes = field.1.iter().map(|attribute|{
                eprintln!("{:?}", attribute.meta.path().get_ident().unwrap().to_string());
                from_attribute_to_comment(attribute.clone())
            });

            quote! {
                #(#attributes)*
                #_field
            }
        });

        let _return = quote!{
            #[doc = "I work here"]
            pub struct #table_name_ident {
                #(#fields)*
            }
        };
        eprintln!("The structure is generated");
        _return
    };
    // create an impl
    impl_table = {
        let fields = construct_table_s.iter().map(|field|{
            if field.1.len() == 0 {
                return quote!{};
            }
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
            #[doc = "I work here"]
            #table_name_ident
            {
                #(#fields)*
            }
        )
    };

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

                quote! {
                    #comments_q
                    pub #field,
                }
            });

            quote! {
                #(#not_t_fields)*
            }
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
            #[crate::impl_table]
            #table_struct

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
#[proc_macro_attribute]
pub fn repo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let entity = parse_macro_input!(_attr as LitStr).value();
    let entity_ident = Ident::new(&*entity, Span::call_site());


    let table = parse_macro_input!(input as DeriveInput);
    let table_name = table.ident;
    let mut table = match table.data {
        Data::Struct(table) => {table}
        Data::Enum(_) => {
            panic!("It should be a struct")
        }
        Data::Union(_) => {
            panic!("It should be a struct")
        }
    };
    let def_struct = {
        let table_fields = table.fields.iter().map(|field| {
            field
        });

        quote! {

            struct #table_name {
                db_connection: Connection,
                entities: Vec<#entity_ident>,
                #(#table_fields), *
            }
        }
    };

    let impl_struct = {
        let connect_to_db = {
            quote! {
                pub fn connect() -> Connection{
                    Connection::open_with_flags(#entity_ident::get_table_name(), OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap()
                }
            }
        };

        let create = {
            quote! {
                pub fn create(&self) -> Result<(), ()> {
                    let struct_d = #entity_ident::default();
                    let send_s = struct_d.get_table2();

                    let mut statement = match self.db_connection.prepare(&*send_s.create()) {
                        Ok(stmt) => stmt,
                        Err(_) => return Err(()),
                    };

                    if let Err(_) = statement.execute([]) {
                        return Err(());
                    }

                    Ok(())
                }
            }
        };

        let insert = {
            quote! {
                pub fn insert(&self, entity: #entity_ident) {
                    let table = entity.get_table2();
                    let (q, v) = table.insert();
                    let mut statement = self.db_connection.prepare(&*q).unwrap();

                    statement.execute(v).unwrap();
                }
            }
        };

        quote! {
            impl #table_name {
                #connect_to_db
                #create
                #insert
            }
        }
    };

    TokenStream::from(quote!{
        use crate::users::*;
        #def_struct
        #impl_struct
    })
}