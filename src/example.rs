
use MyTrait::MyTrait2;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::*;
use Db_shit::HUI::VALUE;
use crate::address::*;
use crate::users::*;
use rusqlite::{Connection, OpenFlags, Row, Rows};
pub mod users {
    use rusqlite::Row;
    use Db_shit::*;
    use crate::address::address;

    pub struct users {
        ///TEXT
        pub text: (DbTypes),
        ///INTEGER_N(10, NEHUI)
        ///PK
        ///AUTO_I
        pub id: (DbTypes, Attributes, Attributes),
    }
    impl users {
        pub fn create(&self) -> String {
            "CREATE TABLE users (\ntext\tTEXT ,\nid\tINTEGER PRIMARY KEY AUTOINCREMENT \n);"
                .to_string()
        }
        pub fn insert(&self) -> (String, (&DbTypes, &DbTypes)) {
            let query =
                    format!(
                        "INSERT INTO {0} ({1}) VALUES ({2});",
                        "users",
                        "text ,id".to_string(),
                        "?, ?", );
            (query, (&self.text, &self.id.0))
        }

        //added
        // create a macro to accept infinitive amount of parameters, and convert it in one
        pub fn load(params: &str) -> String{
            format!("SELECT * from {0}\n{1}",
                    "users", params)
        }


        // in this case HUI should be getted from INTEGER_N not from Address struct, it make api more flexible
        pub fn from_row(row: &Row) -> Self {
            users {
                id: (
                    DbTypes::INTEGER_N(row.get::<&str, HUI<i32>>("id").unwrap()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                text: DbTypes::TEXT(row.get::<&str, String>("text").unwrap()),
            }
        }
    }
    #[derive(Debug)]
    pub struct Users {
        ///INTEGER_N(10, NEHUI)
        ///PK
        ///AUTO_I
        pub id: HUI<i32>,
        ///TEXT
        pub text: String,
        pub some_val: String,
    }
    impl Entity for Users {
        fn get_table_name() -> String {
            "users".to_string()
        }
    }
    impl Users {
        pub fn get_table2(&self) -> users {
            users {
                text: (DbTypes::TEXT(self.text.clone())),
                id: (
                    DbTypes::INTEGER_N(self.id.clone()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
            }
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
//added
    pub fn from_shadow_table(shadow_table: users::users) -> Users {

        let mut users = Users::default();
        //for each field in shadow_staruct
        users.id = match shadow_table.id.0 {
            DbTypes::INTEGER_N(val) => {val}
            _ => panic!("Incorrect type, while parsing from shadow table")
        };
        users.text = match shadow_table.text {
            DbTypes::TEXT(val) => {val}
            _ => panic!("Incorrect type, while parsing from shadow table")
        };

        users
    }
}
pub mod address {
    use rusqlite::Row;
    use Db_shit::*;
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
                        "?, ?",
                    );
            (query, (&self.id.0, &self.address))
        }

        //added
        // create a macro to accept infinitive amount of parameters, and convert it in one
        pub fn load(params: &str) -> String{
                format!("SELECT * from {0}\n{1}",
                        "address", params)
        }


        // in this case HUI should be getted from INTEGER_N not from Address struct, it make api more flexible
        pub fn from_row(row: &Row) -> Self {
            address {
                id: (
                    DbTypes::INTEGER_N(row.get::<&str, HUI<i32>>("id").unwrap()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                address: DbTypes::TEXT(row.get::<&str, String>("address").unwrap()),
            }
        }
    }

    #[derive(Debug)]
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
        //add
        pub fn from_shadow(shadow_table: address) -> Self {
            Address {
                id: match shadow_table.id.0 {
                    // hardcode the value of id, INTEGER_N
                    // later add a transoform function so it can accept a wrapped i32 for example, with realised trait.
                    DbTypes::INTEGER_N(val) => {
                        val
                    },
                    _ => {panic!("Incorrect type, while parsing from shadow table")}
                },
                address: match shadow_table.address {
                    DbTypes::TEXT(val) => {val},
                    _ => {panic!("Incorrect type, while parsing from shadow table")}
                },
            }
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

    //add
    pub fn load(&mut self) {
        let q = users::users::load("");
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement.query_map([], |row: &Row| {
            let a_s = users::users::from_row(row);
            self.entities.push(Users::from_shadow_table(a_s));
            Ok(())
        }).unwrap().for_each(drop);

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

    //add
    pub fn load(&mut self) {
        let q = address::address::load("");
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement.query_map([], |row: &Row| {
            let a_s = address::address::from_row(row);
            self.entities.push(Address::from_shadow(a_s));
            Ok(())
        }).unwrap().for_each(drop);

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
    /*
    let mut user = Users::default();
    user.text = "Bye man".to_string();
    let mut user_r = UserRepo::new();
    user_r.insert(user);
     */

    let mut repo = UserRepo::new();
    repo.load();

    for addr in repo.entities {
        println!("{:?}", addr);
    }
}