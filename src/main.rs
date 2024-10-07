use std::io::ErrorKind::Other;
use p_macros::repo;
use MyTrait::MyTrait2;
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use p_macros::table;
use Db_shit::*;
use Db_shit::OptionalNULL::VALUE;
use crate::users::*;
#[table("users")]
struct Users {
    #[INTEGER_N]
    #[PK]
    #[AUTO_I]
    id: OptionalNULL<i32>,
    #[TEXT]
    text: String,
}

impl Users {
    pub fn default() -> Users {
        Users {
            id: OptionalNULL::NULL,
            text: "".to_string(),
        }
    }
    pub fn new(id: i32, text: String) -> Users {
        Users {
            id: VALUE(id),
            text
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
    let user = Users::new(10, "Hello".to_string());

    let user_repo = UserRepo::new();
    let mut user2 = Users::default();
    user2.text = "I MEAN I DIDN'T EXPACT IT".to_string();
    user_repo.insert(user2);
}
