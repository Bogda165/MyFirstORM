use std::str::EncodeUtf16;
use proc_macro2::Ident;
use quote::quote;
use crate::repo_struct::define_repo_struct::create_struct;

fn create_func(entity_ident: &Ident) -> proc_macro2::TokenStream {
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
}

fn insert_func(entity_ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        pub fn insert(&self, entity: #entity_ident) {
            let table = entity.get_table2();
            let (q, v) = table.insert();
            let mut statement = self.db_connection.prepare(&*q).unwrap();

            statement.execute(v).unwrap();
        }
    }
}

fn connect_to_db_func(entity_ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        pub fn connect() -> Connection{
            Connection::open_with_flags(#entity_ident::get_table_name(), OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap()
        }
    }
}



fn load_func(entity_ident: &Ident, table_ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        pub fn load(&mut self) {
            let q = #table_ident::#table_ident::load("");
            let mut statement = self.db_connection.prepare(&*q).unwrap();
            statement.query_map([], |row: &Row| {
                let a_s = #table_ident::#table_ident::from_row(row);
                self.entities.push(#table_ident::#entity_ident::from_shadow_table(a_s));
                Ok(())
            }).unwrap().for_each(drop);
        }
    }
}


pub(crate) fn impl_repo_struct(entity_ident: &Ident, repo: &Ident, table_ident: &Ident) -> proc_macro2::TokenStream {
    let connect_to_db = connect_to_db_func(entity_ident);

    let create = create_func(entity_ident);

    let insert = insert_func(entity_ident);

    let load = load_func(entity_ident, table_ident);

    quote! {
        impl #repo {
            #connect_to_db
            #create
            #insert
            #load
        }
    }
}