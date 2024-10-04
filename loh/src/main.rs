use p_macros::repo;
use MyTrait::MyTrait2;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use p_macros::table;
use Db_shit::*;
use crate::users::*;
pub mod users {
    use Db_shit::*;
    pub struct users {
        pub id: (DbTypes, Attributes, Attributes),
        pub text: (DbTypes),
    }

    impl users {
        pub fn create(&self) -> String {
            "CREATE TABLE users (\nid\tINTEGER PRIMARY KEY AUTOINCREMENT ,\ntext\tTEXT \n);"
                .to_string()
        }
        pub fn insert(&self) -> (String, (&DbTypes, &DbTypes)) {
            let query = format!(
                "INSERT INTO {0} ({1}) VALUES ({2});",
                "users",
                "id ,text".to_string(),
                "?, ?",
            );
            (query, (&self.id.0, &self.text))
        }
    }
    pub struct Users {
        pub id: i32,
        pub text: String,
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
                    DbTypes::INTEGER(self.id.clone()),
                    Attributes::PK,
                    Attributes::AUTO_I,
                ),
                text: (DbTypes::TEXT(self.text.clone())),
            }
        }
    }
}
impl Users {
    pub fn default() -> Users {
        Users {
            id: 0,
            text: "".to_string(),
        }
    }
    pub fn new(id: i32, text: String) -> Users {
        Users { id, text }
    }
}
use crate::users::*;
use rusqlite::{Connection, OpenFlags};
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
        println!("{q}");
        let mut statement = self.db_connection.prepare(&*q).unwrap();
        statement.execute(v).unwrap();
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
fn main() {
    let user = Users::new(-1, "Hello".to_string());
    let user_repo = UserRepo::new();
    user_repo.insert(user);
}