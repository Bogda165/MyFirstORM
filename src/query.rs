use std::marker::PhantomData;
use crate::column::Allowed;

struct Query<AllowedTables> {
    tables: PhantomData<AllowedTables>,
}

impl<AllowedTables> Query<AllowedTables> {
    fn add_table<Table>(self) -> Query<(Table, AllowedTables)> {
        Query {
            tables: PhantomData::<(Table, AllowedTables)>,
            //basic impl
        }
    }
}