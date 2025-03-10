extern crate alloc;

use orm_traits::db::OrmDataBase;
use dsl::column::{Allowed, Table};
use dsl::query::the_query::Query;
use dsl::convertible::TheType;
use dsl::{from, from_tables, query_from};
use dsl::queryable::Queryable;
use dsl::safe_expressions::{column, literal};
use p_macros::{repo, attrs_to_comments, data_base};
use p_macros::table;
use rusqlite::{params, Error, Params, Row};
use orm_traits::attributes::*;

use crate::address::*;
use crate::users::*;
use p_macros::OrmTable;
use orm_traits::{OrmColumn, OrmTable};
use rusqlite::{Connection, OpenFlags};
use rusqlite::types::Value;
use std::any::Any;

use load_logic::code::*;

#[derive(Default, Clone)]
#[table("users")]
struct Users {
    #[column]
    #[sql_type(Int)]
    #[constraint(PrimaryKey)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub text: String,
    #[connect(Address, address_id)]
    #[connect_type(OneToOne)]
    pub addr: Address,
    pub some_val_that_wont_be_used_in_db: String,
}

// if One to One create a field address_id, if the connection
// if One to Many create nothing

//#[connect_many(Address, "user_id")]
//#[connect_many_to_many(UsersAddress, Address, "address_id")]
//#[sql_type(Int)]
impl Users {
    pub fn default() -> Users {
        Users {
            id: 0,
            text: "".to_string(),
            addr: Address::default(),
            some_val_that_wont_be_used_in_db: "NotNull".to_string()
        }
    }
    pub fn new(_id: i32, _text: String, some_val: String) -> Users {
        Users {
            addr: Address::default(),
            id: _id,
            text: _text,
            some_val_that_wont_be_used_in_db: some_val,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[p_macros::table("address")]
struct Address {
    #[column]
    #[sql_type(Int)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub _address: String,
}


impl Address {
    pub fn new(addr: String) -> Self {
        Address {
            id: 0,
            _address: addr,
        }
    }

    pub fn default() -> Self {
        Address {
            id: 0,
            _address: "".to_string()
        }
    }
}

#[data_base(users::Users, address::Address)]
#[name = "Test_db"]
#[from(users::Users, address::Address)]
struct DataBaseTest {

}


fn main() {

    let users = Users::from_columns((10, "name".to_string()));

    let query = query_from!(users::Users).join::<address::Address>(literal(10).less(column(address::id))).select_test(((column(users::id), "hui"), (column(address::id), "hui2")));

    let _id = users::id;
    println!("Create query {}", address::Address::create_query());
    println!("Load query: {}", query.to_query());


    let mut db_connection = DataBaseTest::default();
    db_connection.connect();

    let _vec = vec![Address {
        id: 10,
        _address: "one stree".to_string(),
    }, Address {id: 11, _address: "another street".to_string()}];

    Address::insert_iterator(_vec.into_iter(), &mut db_connection);

    let select_q = query_from!(Address).select_test(((column(address::id), "hui2"), (column(address::_address), "hui3")));

    println!("{}", select_q.to_query());

    db_connection.query_get(&*select_q.to_query(), |row| {
        row.get::<usize, String>(1).unwrap()
    }).into_iter().for_each(|result| {println!("HUI{:?}", result)})

}