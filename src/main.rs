extern crate alloc;

use orm_traits::db::OrmDataBase;
use dsl::column::{Allowed, Table};
use dsl::query::the_query::Query;
use dsl::convertible::TheType;
use dsl::{from, from_tables, query_from};
use dsl::queryable::Queryable;
use dsl::safe_expressions::{column, literal};
use Db_shit::Entity;
use p_macros::{repo, impl_table, attrs_to_comments, data_base};
use p_macros::table;
use rusqlite::{params, Error, Params, Row};
use rusqlite::ffi::{SQLITE_OPEN_CREATE, SQLITE_OPEN_READWRITE};
use Db_shit::{Attributes, DbTypes};
use Db_shit::NotNull;
use orm_traits::attributes::*;

use crate::address::*;
use crate::users::*;
use p_macros::OrmTable;
use orm_traits::{OrmColumn, OrmTable};
use rusqlite::{Connection, OpenFlags};
use rusqlite::ErrorCode::DatabaseBusy;
use rusqlite::types::Type;
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
use orm_traits::repo::OrmRepo;

#[derive(Default)]
struct AddrRepo {
    db_connection: Option<Connection>,
    entities: Vec<Address>,
}

impl AddrRepo {

}

impl OrmRepo<Address> for AddrRepo {
    fn from_connection(connection: Connection) -> Self {
        let mut ar = AddrRepo::default();
        ar.db_connection = Some(connection);
        ar
    }
    fn connect() -> Self
        where Self:Sized
    {
        Self::from_connection(
            Connection::open_with_flags(
                "Test_db".to_string(),
                OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
            ).unwrap()
        )
    }

    fn get_connection(&self) -> &Option<Connection> {
        &self.db_connection
    }
}

#[data_base(users::Users, address::Address)]
#[name = "Test_db"]
#[from(users::Users, address::Address)]
struct DataBaseTest {

}

fn main() {

    let users = Users::from_columns((10, "name".to_string()));

    let query = query_from!(users::Users).join::<address::Address>(literal(10).less(column(address::id))).select_test((column(users::id), column(address::id)));

    let _id = users::id;
    println!("Inset query {:?}",users.insert_query());
    println!("Create query {}", address::Address::create_query());
    println!("Load query: {}", query.to_query());


    let insert_query = Address {
        id: 10,
        _address: "prospekt huia".to_string(),
    }.insert_query();

    let mut db_connection = DataBaseTest::default();
    db_connection.connect();

    // let binding = db_connection.connection.unwrap();
    // let mut binding = binding.prepare("select * from Address;").unwrap();
    // let mut rows = binding.query([]).unwrap();
    //
    // while let Ok(row) = rows.next() {
    //     println!("{}", row.unwrap().get::<usize, String>(1).unwrap());
    // }



    let select_q = query_from!(Address).select_test((column(address::id), column(address::_address)));

    println!("{}", select_q.to_query());

    db_connection.query_get(&*select_q.to_query(), |row| {
        row.get::<usize, String>(1).unwrap()

    }).into_iter().for_each(|result| {println!("HUI{:?}", result)})
   }