use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::DeriveInput;


fn from_connection_quote(repo_name: &Ident) -> TokenStream2 {
    quote! {
        fn from_connection(connection: Connection) -> Self {
            let mut ar = #repo_name::default();
            ar.db_connection = Some(connection);
            ar
        }
    }
}

fn connect_fn_quote(db_name: String) -> TokenStream2 {
    quote! {
        fn connect() -> Self
            where Self:Sized
        {
            Self::from_connection(
                Connection::open_with_flags(
                    #db_name.to_string(),
                    OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                ).unwrap()
            )
        }
    }
}

fn get_connection_fn_quote() -> TokenStream2 {
    quote! {
        fn get_connection(&self) -> &Option<Connection> {
            &self.db_connection
        }
    }
}


pub fn orm_repo_derive_f(repo: DeriveInput, table: Ident, db_name: String ) -> TokenStream2 {
    quote!{}
}