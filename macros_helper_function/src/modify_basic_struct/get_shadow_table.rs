use std::collections::HashMap;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;

// that mf should be ident
pub(crate) fn inside_db_type_fn(input: proc_macro2::TokenStream ) -> proc_macro2::TokenStream {
    quote! {
        self.#input.clone()
    }
}