use MyTrait::MyTrait2;
use rusqlite::{Connection, OpenFlags};
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use p_macros::table;
use Db_shit::*;


 #[table("users")]
struct Users {
    #[INTEGER]
    #[PK]
    #[AUTO_I]
    id: i32,
    #[TEXT]
    text: String,
}

fn main() {

    let conn = Connection::open_with_flags("FirstDb", OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();

    let user = user{
        id: 10,
        name: "hello".to_string()
    };

    let loh = user.get_table();
    println!("{:?}", loh.name);

    let mut table = users::Users {
        id: 10,
        text: "My name is lOH".to_string(),
    };

    table.id = 11;

    let mut prep = conn.prepare(&*table.get_table2().create()).unwrap();
    prep.execute(()).unwrap();
}
