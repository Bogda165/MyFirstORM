use crate::Attributes::{AUTO_I, PK};
use macros_l::*;
use proc_macro2::Span;
use rusqlite::types::Value::Null;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Connection, ToSql};

trait Insertable {
    fn for_insert(&self, num: i32) -> String;
}

#[derive(Debug, Clone)]
pub enum NotNull<T> {
    NULL,
    VALUE(T),
}

impl<T> ToSql for NotNull<T>
where
    T: ToSql,
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            NotNull::NULL => Null.to_sql(),
            NotNull::VALUE(val) => val.to_sql(),
        }
    }
}

impl FromSql for NotNull<i32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(NotNull::NULL),
            ValueRef::Integer(val) => {
                //TODO fix to i64 uoy
                Ok(NotNull::VALUE(val as i32))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for NotNull<f32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(NotNull::NULL),
            ValueRef::Real(val) => Ok(NotNull::VALUE(val as f32)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for NotNull<String> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(NotNull::NULL),
            ValueRef::Text(val) => Ok(NotNull::VALUE(
                std::str::from_utf8(val).unwrap().to_string(),
            )),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug)]
#[fields_name]
pub enum Attributes {
    PK,
    AUTO_I,
    CONNECT(&'static str),
}

impl Attributes {
    pub fn to_string(&self) -> String {
        match &self {
            PK => "PRIMARY KEY".to_string(),
            AUTO_I => "AUTOINCREMENT".to_string(),
            Attributes::CONNECT(_) => "CONNECT IDK)".to_string(),
        }
    }
}

pub trait TableTrait {}
//create additional array for many to many connection, add it in the end of the query, if needed, as different query

// a repore represent a list of table types, which must realise method get table(shadow table)
/*
   -CREATE TABLE person (
   ident.to_sring() for field in fields field.to_string()
                 id              INTEGER PRIMARY KEY,
                 name            TEXT NOT NULL,
                 age             INTEGER
               )",
*/
/*

   INSERT INTO person (name, age) VALUES (?1, ?2)
*/

pub trait Entity {
    fn get_table_name() -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::__private::ToTokens;

    #[test]
    fn test_get_types() {
        TestEnum::get_variants().iter().for_each(|ty| {
            println! {"{}", quote!{#ty}};
        })
    }
}
