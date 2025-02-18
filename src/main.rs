extern crate alloc;
use Db_shit::Entity;
use p_macros::{repo, impl_table};
use p_macros::table;
use rusqlite::Row;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::{Attributes, DbTypes};
use Db_shit::NotNull;

use crate::address::*;
use crate::users::*;
use rusqlite::{Connection, OpenFlags};
use Db_shit::NotNull::VALUE;

#[table("users")]
struct Users {
    #[INTEGER_N(10, NEHUI)]
    #[PK]
    #[AUTO_I]
    id: NotNull<i32>,
    #[TEXT]
    text: String,
    some_val: String,
}
impl Users {
    pub fn default() -> Users {
        Users {
            id: NotNull::NULL,
            text: "".to_string(),
            some_val: "NotNull".to_string()
        }
    }
    pub fn new(_id: i32, _text: String, some_val: String) -> Users {
        Users {
            id: VALUE(_id),
            text: _text,
            some_val,
        }
    }
}

#[derive(Debug)]
#[p_macros::table("address")]
struct Address {
    #[INTEGER_N]
    #[PK]
    #[AUTO_I]
    #[constraints(Int, PK)]
    id: NotNull<i32>,
    #[TEXT]
    _address: String,
}


impl Address {
    pub fn new(addr: String) -> Self {
        Address {
            id: NotNull::NULL,
            _address: addr,
        }
    }

    pub fn default() -> Self {
        Address {
            id: NotNull::NULL,
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

    // let address = Address::new("Bal 20".to_string());
    // let mut a_r = AddrRepo::new();
    // a_r.insert(Address { id: NotNull::VALUE(10), address: "Some address".to_string() })

}