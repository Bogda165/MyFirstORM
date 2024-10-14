use std::io::ErrorKind::Other;
use p_macros::{repo, table, impl_table};
use MyTrait::MyTrait2;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::*;
use Db_shit::OptionalNULL::VALUE;
use crate::address::*;
use crate::users::*;
use rusqlite::{Connection, OpenFlags};
#[table("users")]
struct Users {
    #[INTEGER_N(10, NEHUI)]
    #[PK]
    #[AUTO_I]
    id: OptionalNULL<i32>,
    #[TEXT]
    text: String,
    some_val: String,
}

impl Users {
    pub fn default() -> Users {
        Users {
            id: OptionalNULL::NULL,
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

#[table("address")]
struct Address {
    #[INTEGER_N]
    #[PK]
    #[AUTO_I]
    id: OptionalNULL<i32>,
    #[TEXT]
    address: String,

}

impl Address {
    pub fn new(addr: String) -> Self {
        Address {
            id: OptionalNULL::NULL,
            address: addr,
        }
    }

    pub fn default() -> Self {
        Address {
            id: OptionalNULL::NULL,
            address: "".to_string()
        }
    }
}

#[repo("Users")]
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

#[repo("Address")]
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

    // let conn = Connection::open_with_flags("FirstDb", OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
    //
    // let user = user{
    //     id: 10,
    //     name: "hello".to_string()
    // };
    //
    // let loh = user.get_table();
    // println!("{:?}", loh.name);
    //
    // let mut table = users::Users {
    //     id: 10,
    //     text: "My name is lOH".to_string(),
    // };
    //
    // table.id = 11;
    //
    // let mut prep = conn.prepare(&*table.get_table2().create()).unwrap();
    // prep.execute(()).unwrap();
    let address = Address::new("Bal 20".to_string());
    let mut a_r = AddrRepo::new();
    a_r.create().unwrap();
    a_r.insert(address);
}
