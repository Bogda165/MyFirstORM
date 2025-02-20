extern crate alloc;
use Db_shit::Entity;
use p_macros::{repo, impl_table, attrs_to_comments};
use p_macros::table;
use rusqlite::Row;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::{Attributes, DbTypes};
use Db_shit::NotNull;
use orm_traits::attributes::*;

use crate::address::*;
use crate::users::*;
use p_macros::OrmTable;
use orm_traits::{OrmColumn, OrmTable};
use rusqlite::{Connection, OpenFlags};
use Db_shit::NotNull::VALUE;

#[derive(Default)]
#[table("users")]
struct Users {
    #[column]
    #[sql_type(Int)]
    #[constraint(PrimaryKey)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub text: String,
    pub some_val: String,
}
impl Users {
    pub fn default() -> Users {
        Users {
            id: 0,
            text: "".to_string(),
            some_val: "NotNull".to_string()
        }
    }
    pub fn new(_id: i32, _text: String, some_val: String) -> Users {
        Users {
            id: _id,
            text: _text,
            some_val,
        }
    }
}

#[derive(Debug, Default)]
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

// #[repo(entity = "Users", table = "users")]
// struct UserRepo {
//
// }
//
//
// impl UserRepo {
//     pub fn new() -> Self {
//         UserRepo {
//             db_connection: UserRepo::connect(),
//             entities: Vec::new(),
//         }
//     }
// }
//
// #[repo(table = "address", entity = "Address")]
// struct AddrRepo {
//
// }
//
// impl AddrRepo {
//     pub fn new() -> Self {
//         AddrRepo {
//             db_connection: AddrRepo::connect(),
//             entities: Vec::new(),
//         }
//     }
// }

fn main() {

    let users = Users::from_columns((10, "name".to_string()));

    println!("Inset query {:?}",users.insert_query());
    println!("Create query {}", users::Users::create_query());

}