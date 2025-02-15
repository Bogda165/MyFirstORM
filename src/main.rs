use std::collections::HashMap;
use Db_shit::Entity;
use std::io::ErrorKind::Other;
use p_macros::{repo, table, impl_table};
use MyTrait::MyTrait2;
use rusqlite::{Row, Statement};
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::{Attributes, DbTypes};
use Db_shit::HUI;
use crate::address::*;
use crate::users::*;
use rusqlite::{Connection, OpenFlags};
use Db_shit::HUI::VALUE;

struct TableName {
    name: String
}

struct EntityQuery<'a> {
    query: Statement<'a>,
    entity_queries: HashMap<TableName, EntityQuery<'a>>
}

trait ConvertableFromEntityQuery {
    fn load() {

    }
}

pub mod users {
    use Db_shit::*;
    use rusqlite::Row;
    pub struct users {
        ///INTEGER_N(10, NEHUI)
        ///PK
        ///AUTO_I
        pub id: (DbTypes, Attributes, Attributes),
        ///TEXT
        pub text: (DbTypes),
    }
    impl users {
        pub fn create(&self) -> String {
            "CREATE TABLE users (\nid\tINTEGER PRIMARY KEY AUTOINCREMENT ,\ntext\tTEXT \n);"
                .to_string()
        }
        pub fn insert(&self) -> (String, (&DbTypes, &DbTypes)) {
            let query =
                    format!(
                        "INSERT INTO {0} ({1}) VALUES ({2});",
                        "users",
                        "id ,text".to_string(),
                        "?, ?",
                   );
            (query, (&self.id.0, &self.text))
        }
        pub fn load(params: &str) -> String {
                format!("SELECT * from {0}\n{1}", "users", params)
        }
        pub fn from_row(row: &Row) -> Self {
            users {
                id: (
                    DbTypes::INTEGER_N(row.get::<&str, HUI<i32>>("id").unwrap()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                text: (DbTypes::TEXT(row.get::<&str, String>("text").unwrap())),
            }
        }
    }

    ///users
    pub struct Users {
        ///INTEGER_N(10, NEHUI)
        ///PK
        ///AUTO_I
        pub id: HUI<i32>,
        ///TEXT
        pub text: String,
        pub some_val: String,
        ///FK("Address")
        pub
    }
    impl Entity for Users {
        fn get_table_name() -> String {
            "users".to_string()
        }
    }
    impl Users {
        pub fn get_table2(&self) -> users {
            users {
                id: (
                    DbTypes::INTEGER_N(self.id.clone()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                text: (DbTypes::TEXT(self.text.clone())),
            }
        }
        pub fn from_shadow_table(shadow_table: users) -> Self {
            let mut users = Users::default();
            users.id = match shadow_table.id.0 {
                DbTypes::INTEGER_N(val) => val,
                _ => {
                        panic!("Incorrect type, while parsing from shadow table"
                    );
                }
            };
            users.text = match shadow_table.text {
                DbTypes::TEXT(val) => val,
                _ => {
                        panic!("Incorrect type, while parsing from shadow table"
                    );
                }
            };
            users
        }
    }
}
impl Users {
    pub fn default() -> Users {
        Users {
            id: HUI::NULL,
            text: "".to_string(),
            some_val: "HUI".to_string(),
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
pub mod address {
    use Db_shit::*;
    use rusqlite::Row;
    pub struct address {
        ///INTEGER_N
        ///PK
        ///AUTO_I
        pub id: (DbTypes, Attributes, Attributes),
        ///TEXT
        pub address: (DbTypes),
    }
    impl address {
        pub fn create(&self) -> String {
            "CREATE TABLE address (\nid\tINTEGER PRIMARY KEY AUTOINCREMENT ,\naddress\tTEXT \n);"
                .to_string()
        }
        pub fn insert(&self) -> (String, (&DbTypes, &DbTypes)) {
            let query =
                    format!(
                        "INSERT INTO {0} ({1}) VALUES ({2});",
                        "address",
                        "id ,address".to_string(),
                        "?, ?"
            );
            (query, (&self.id.0, &self.address))
        }
        pub fn load(params: &str) -> String {
            format!("SELECT * from {0}\n{1}", "address", params)
        }
        pub fn from_row(row: &Row) -> Self {
            address {
                id: (
                    DbTypes::INTEGER_N(row.get::<&str, HUI<i32>>("id").unwrap()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                address: (DbTypes::TEXT(row.get::<&str, String>("address").unwrap())),
            }
        }
    }
    #[derive(Debug)]
    ///address
    pub struct Address {
        ///INTEGER_N
        ///PK
        ///AUTO_I
        pub id: HUI<i32>,
        ///TEXT
        pub address: String,
    }
    impl Entity for Address {
        fn get_table_name() -> String {
            "address".to_string()
        }
    }
    impl Address {
        pub fn get_table2(&self) -> address {
            address {
                id: (
                    DbTypes::INTEGER_N(self.id.clone()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                address: (DbTypes::TEXT(self.address.clone())),
            }
        }
        pub fn from_shadow_table(shadow_table: address) -> Self {
            let mut address = Address::default();
            address.id = match shadow_table.id.0 {
                DbTypes::INTEGER_N(val) => val,
                _ => {
                        panic!("Incorrect type, while parsing from shadow table"
                    );
                }
            };
            address.address = match shadow_table.address {
                DbTypes::TEXT(val) => val,
                _ => {
                        panic!("Incorrect type, while parsing from shadow table"
                    );
                }
            };
            address
        }
    }
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
            address: "".to_string(),
        }
    }
}
use crate::users::*;
struct UserRepo {
    db_connection: Connection,
    entities: Vec<Users>,
}
impl UserRepo {
    pub fn connect() -> Connection {
        Connection::open_with_flags(
            Users::get_table_name(),
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
            .unwrap()
    }
    pub fn create(&self) -> Result<(), ()> {
        let struct_d = Users::default();
        let send_s = struct_d.get_table2();
        let mut statement = match self.db_connection.prepare(&*send_s.create()) {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };
        if let Err(_) = statement.execute([]) {
            return Err(());
        }
        Ok(())
    }
    pub fn insert(&self, entity: Users) {
        let table = entity.get_table2();
        let (q, v) = table.insert();
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement.execute(v).unwrap();
    }
    pub fn load(&mut self) {
        let q = users::users::load("");
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement
            .query_map(
                [],
                |row: &Row| {
                    let a_s = users::users::from_row(row);
                    self.entities.push(users::Users::from_shadow_table(a_s));
                    Ok(())
                },
            )
            .unwrap()
            .for_each(drop);
    }
}
impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            db_connection: UserRepo::connect(),
            entities: Vec::new(),
        }
    }
}
use crate::users::*;
struct AddrRepo {
    db_connection: Connection,
    entities: Vec<Address>,
}
impl AddrRepo {
    pub fn connect() -> Connection {
        Connection::open_with_flags(
            Address::get_table_name(),
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
            .unwrap()
    }
    pub fn create(&self) -> Result<(), ()> {
        let struct_d = Address::default();
        let send_s = struct_d.get_table2();
        let mut statement = match self.db_connection.prepare(&*send_s.create()) {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };
        if let Err(_) = statement.execute([]) {
            return Err(());
        }
        Ok(())
    }
    pub fn insert(&self, entity: Address) {
        let table = entity.get_table2();
        let (q, v) = table.insert();
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement.execute(v).unwrap();
    }
    pub fn load(&mut self) {
        let q = address::address::load("");
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement
            .query_map(
                [],
                |row: &Row| {
                    let a_s = address::address::from_row(row);
                    self.entities.push(address::Address::from_shadow_table(a_s));
                    Ok(())
                },
            )
            .unwrap()
            .for_each(drop);
    }
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
    a_r.create().unwrap();
    a_r.load();
    for i in a_r.entities {
        println!("{0:?}\n", i);
    }
}