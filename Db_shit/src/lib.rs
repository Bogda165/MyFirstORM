use std::collections::HashMap;
use syn::parse2;
use syn::TypePath;
use syn::Meta::Path;
use syn::PathSegment;
use syn::Ident;
use syn::Type::Paren;
use syn::TypeTuple;
use syn::Type;
use std::any::type_name;
use std::error::Error;
use macros_l::*;
use rusqlite::{Connection, ToSql};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::types::Value::Null;
use proc_macro2::{Span};
use syn::__private::quote::quote;
use crate::Attributes::{AUTO_I, PK};
use crate::DbTypes::*;

trait Insertable {
    fn for_insert(&self, num: i32) -> String;
}

#[derive(Debug, Clone)]
pub enum NotNull<T> {
    NULL,
    VALUE(T)
}

impl<T> ToSql for NotNull<T>
    where T: ToSql
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            NotNull::NULL => {Null.to_sql()}
            NotNull::VALUE(val) => {
                val.to_sql()
            }
        }
    }
}

impl FromSql for NotNull<i32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(NotNull::NULL)
            }
            ValueRef::Integer(val) => {
                //TODO fix to i64 uoy
                Ok(NotNull::VALUE(val as i32))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for NotNull<f32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(NotNull::NULL)
            }
            ValueRef::Real(val) => {
                Ok(NotNull::VALUE(val as f32))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for NotNull<String> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(NotNull::NULL)
            }
            ValueRef::Text(val) => {
                Ok(NotNull::VALUE(std::str::from_utf8(val).unwrap().to_string()))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

#[fields_name]
#[get_types]
#[derive(Debug)]
pub enum DbTypes {
    INTEGER_N(NotNull<i32>),
    FLOAT_N(NotNull<f32>),
    TEXT_N(NotNull<String>),
    INTEGER(i32),
    FLOAT(f32),
    TEXT(String),
}

impl DbTypes {
    pub fn to_string(&self) -> String {
        match &self {
            INTEGER(_) => {
                "INTEGER".to_string()
            }
            FLOAT(_) => {
                "REAL".to_string()
            }
            TEXT(_) => {
                "TEXT".to_string()
            }
            INTEGER_N(_) => {
                "INTEGER".to_string()
            }
            FLOAT_N(_) => {
                "REAL".to_string()
            }
            TEXT_N(_) => {
                "TEXT".to_string()
            }

            _ => {
                unreachable!("I hae not any others types")
            }
        }
    }
}

impl Insertable for DbTypes {
    fn for_insert(&self, num: i32) -> String {
        match self {
            INTEGER(val) => {
                val.to_string()
            }
            FLOAT(val) => {
                val.to_string()
            }
            TEXT(str) => {
                str.clone()
            }
            INTEGER_N(val) => {
                match val {
                    NotNull::NULL => {"NULL".to_string()}
                    NotNull::VALUE(val) => {val.to_string()}
                }
            }
            FLOAT_N(val) => {
                match val {
                    NotNull::NULL => {"NULL".to_string()}
                    NotNull::VALUE(val) => {val.to_string()}
                }
            }
            TEXT_N(val) => {
                match val {
                    NotNull::NULL => {"NULL".to_string()}
                    NotNull::VALUE(val) => {val.clone()}
                }
            }
        }
    }
}


impl ToSql for DbTypes {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            DbTypes::INTEGER(i) => {Ok(ToSqlOutput::from(*i))},
            DbTypes::FLOAT(f) => Ok(ToSqlOutput::from(*f)),
            DbTypes::TEXT(s) => Ok(ToSqlOutput::from(s.clone())),
            INTEGER_N(i) => {i.to_sql()},
            FLOAT_N(f) => {f.to_sql()},
            TEXT_N(t) => {t.to_sql()},
        }
    }
}

impl FromSql for DbTypes {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Err(FromSqlError::InvalidType)
            }
            ValueRef::Integer(val) => {
                Ok(INTEGER(val as i32))
            }
            ValueRef::Real(val) => {
                Ok(FLOAT(val as f32))
            }
            ValueRef::Text(val) => {
                Ok(TEXT(std::str::from_utf8(val).unwrap().to_string()))
            }
            ValueRef::Blob(_) => {unreachable!("Fuck it")}
        }
    }
}

#[derive(Debug)]
#[fields_name]
pub enum Attributes {
    PK,
    AUTO_I,
    CONNECT(&'static str)
}

impl Attributes {
    pub fn to_string(&self) -> String {
        match &self {
            PK => {
                "PRIMARY KEY".to_string()
            }
            AUTO_I => {
                "AUTOINCREMENT".to_string()
            }
            Attributes::CONNECT(_) => {
                "CONNECT IDK)".to_string()
            }
        }
    }

}


pub trait TableTrait {

}
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

pub trait Entity
{
    fn get_table_name() -> String;
}

#[fields_name]
#[get_types]
enum TestEnum {
    VALUE(i32),
    VALUE2(NotNull<i32>),
    VALUE3(String, i32, f32),
}


#[cfg(test)]
mod tests {
    use syn::__private::ToTokens;
    use super::*;

    /*#[test]
    fn test_get_table() {
       /* let user_instance = user {
            id: 1,
            name: "Alice".to_string(),
        };

        let table = user_instance.get_table();

        match table.id {
            (DbTypes::INTEGER(val), Attributes::PK, Attributes::AUTO_I) => assert_eq!(val, 1),
            _ => panic!("ID field did not match expected values"),
        }

        match table.name {
            DbTypes::TEXT(val) => assert_eq!(val, "Alice".to_string()),
            _ => panic!("Name field did not match expected values"),
        }

        */
    }*/

    #[test]
    fn test_get_types() {
        TestEnum::get_variants().iter().for_each(|ty| {
            println!{"{}", quote!{#ty}};
        })
    }
}