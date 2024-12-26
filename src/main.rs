use Db_shit::Entity;
use std::io::ErrorKind::Other;
use p_macros::{repo, table, impl_table};
use MyTrait::MyTrait2;
use rusqlite::Row;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::{Attributes, DbTypes};
use Db_shit::HUI;

use crate::address::*;
use crate::users::*;
use rusqlite::{Connection, OpenFlags};
use Db_shit::HUI::VALUE;

#[table("users")]
struct Users {
    #[INTEGER_N(10, NEHUI)]
    #[PK]
    #[AUTO_I]
    id: HUI<i32>,
    #[TEXT]
    text: String,
    some_val: String,
}

impl Users {
    pub fn default() -> Users {
        Users {
            id: HUI::NULL,
            text: "".to_string(),
            some_val: "HUI".to_string()
        }
    }
    pub fn new(id: i32, text: String, some_val: String) -> Users {
        Users {
            id: VALUE(id),
            text,
            some_val,
        }
    }
}

#[derive(Debug)]
#[table("address")]
struct Address {
    #[INTEGER_N]
    #[PK]
    #[AUTO_I]
    id: HUI<i32>,
    #[TEXT]
    address: String,

}

impl Address {
    pub fn new(addr: String) -> Self {
        Address {
            id: HUI::NULL,
            address: addr,
        }
    }

    pub fn default() -> Self {
        Address {
            id: HUI::NULL,
            address: "".to_string()
        }
    }
}

#[repo(entity = "Users", table = "users")]
struct UserRepo {

}


impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            db_connection: UserRepo::connect(),
            entities: Vec::new(),
        }
    }
}

#[repo(table = "address", entity = "Address")]
struct AddrRepo {

}

impl AddrRepo {
    pub fn new() -> Self {
        AddrRepo {
            db_connection: AddrRepo::connect(),
            entities: Vec::new(),
        }
    }
}

fn main() {

    let address = Address::new("Bal 20".to_string());
    let mut a_r = AddrRepo::new();
    a_r.load();

    for i in a_r.entities {
        println!("{:?}", i);
    }

}