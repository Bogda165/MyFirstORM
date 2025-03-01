use std::any::Any;
use dsl::column::{Column, RawColumn, Table};
use dsl::convertible::TheType;
use dsl::expressions::Expression::Raw;
use dsl::expressions::raw_types::RawTypes;
use rusqlite::types::FromSql;
use crate::join::Join;

pub mod join {
    pub trait Join
    where
        Self: Iterator
    {
        fn join_iter(self, divide: &str) -> String;
    }

    impl<I: Iterator + Sized> Join for I
    where
        <I as Iterator>::Item: Into<String>
    {
        fn join_iter(mut self, divide: &str) -> String {
            match self.next() {
                None => { "".to_string() }
                Some(el) => {
                    self.fold(el.into(), |str1, str2| format!("{}{}{}", str1, divide, str2.into()))
                }
            }
        }
    }

    #[test]
    fn test_join() {
        let str = vec!["name", "users", "id"].into_iter().join_iter(", ");
        assert_eq!(str, "name, users, id".to_string())
    }

    #[test]
    fn empty_join() {
        let str = vec!["some_val"].into_iter().join_iter(", ");
        assert_eq!(str, "some_val".to_string())
    }
}


pub struct OrmColumn {
    pub name: String,
    pub attrs: Vec<String>
}

impl Into<String> for OrmColumn{
    fn into(self) -> String {
        format!("{} {}", self.name, self.attrs.join(" "))
    }
}


impl<T: Column> From<T> for OrmColumn {
    fn from(_: T) -> Self {
        OrmColumn {
            name: T::get_name(),
            attrs: vec![],
        }
    }
}

/// You must not implement this trait explicitly
pub trait OrmTable: Table + Default + FromSql
{
    type ColumnsT;
    fn columns(self) -> Self::ColumnsT;
    /// Return a vec of all columns with their attributes, do not realise by yourself
    fn columns_strings() -> Vec<OrmColumn>;

    fn create_query() -> String {
        format!("CREATE TABLE {} ({})",
                Self::get_name(),
                Self::columns_strings().into_iter().join_iter(", ")
        )
    }

    fn insert_query(self) -> (String, Self::ColumnsT)
    where Self: Sized
    {
        let query =
            format!("INSERT INTO {0} ({1}) VALUES ({2});",
                Self::get_name(),
                Self::columns_strings().into_iter().map(|column|{column.name}).join_iter(", "),
                Self::columns_strings().iter().map(|_| {"?".to_string()}).join_iter(", "),
            );
        (query, self.columns())
    }

    fn from_columns(columns: Self::ColumnsT) -> Self
    where
        Self:Sized
    {
        Self::default()
    }
}

pub mod repo {
    use rusqlite::{Connection, Error, OpenFlags, Params};
    use super::*;
    pub trait OrmRepo<T: OrmTable>
    where
        Self: Default,
    {
        fn from_connection(connection: Connection) -> Self;
        fn connect() -> Self
        where
            Self: Sized
        {
            Self::from_connection(
                Connection::open_with_flags(
                    T::get_name(),
                    OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                ).unwrap()
            )
        }

        fn get_connection(&self) -> &Option<Connection>;
        fn create(&self) -> Result<(), Error> {
            match self.get_connection() {
                None => {
                    Err(Error::InvalidQuery)
                }
                Some(ref connection) => {
                    let mut statement = match connection.prepare(&*T::create_query()) {
                        Ok(stmt) => stmt,
                        Err(error) => return Err(error),
                    };
                    if let Err(error) = statement.execute([]) {
                        return Err(error);
                    }
                    Ok(())
                }
            }
        }
        fn insert(&self, entity: T)
        where
            <T as OrmTable>::ColumnsT: Params
        {
            let (q, v) = entity.insert_query();
            match self.get_connection() {
                None => {panic!("table was not connected")}
                Some(connection) => {
                    let mut statement = connection.prepare(&*q).unwrap();
                    statement.execute(v).unwrap();
                }
            }

        }

        // pub fn load(&mut self) {
        //     let q = address::address::load("");
        //     let mut statement = self.db_connection.prepare(&*q).unwrap();
        //     statement
        //         .query_map(
        //             [],
        //             |row: &Row| {
        //                 let a_s = address::address::from_row(row);
        //                 self.entities.push(address::Address::from_shadow_table(a_s));
        //                 Ok(())
        //             },
        //         )
        //         .unwrap()
        //         .for_each(drop);
        // }
    }
}

pub mod db {
    use std::os::macos::raw::stat;
    use dsl::convertible::Conversation;
    use dsl::query::the_query::Query;
    use rusqlite::{Connection, ErrorCode, OpenFlags, Row, Rows, Statement};
    use rusqlite::ffi::Error;

    pub trait OrmDataBase: Default{
        fn get_connection(&self) -> &Option<Connection>;

        fn get_connection_mut(&mut self) -> &mut Option<Connection>;

        fn query_get<F, T>(& self, query: &str, mut clos: F)  -> Result<Vec<T>, rusqlite::Error>
        where
            F: FnMut(&Row) -> T,
        {

            match self.get_connection() {
                None => {
                    Err(rusqlite::Error::SqliteFailure(Error::new(14), Some("Probably the connection was not created".to_string())))
                }
                Some(connection ) => {
                    let mut statement = connection.prepare(query)?;
                    let mut rows = statement.query([])?;
                    let mut clos_result = vec![];

                    while let Ok(row) = rows.next() {
                        match row {
                            None => {break}
                            Some(row) => {clos_result.push(clos(row));}
                        }
                    }

                    Ok(clos_result)
                }
            }
        }

        fn query_post<QParams: rusqlite::Params>(&self, query: &str, params: QParams) -> Result<(), rusqlite::Error> {
            match self.get_connection() {
                None => {
                    Err(rusqlite::Error::SqliteFailure(Error::new(14), Some("Probably the connection was not created".to_string())))
                }
                Some(connection ) => {
                    let mut statement = connection.prepare(query)?;
                    let _ = statement.query(params)?;
                    Ok(())
                }
            }
        }

        fn get_name() -> String;

        fn connect(&mut self) {
            if self.get_connection().is_some() { return }

            *self.get_connection_mut() = Some(
                Connection::open_with_flags(
                    Self::get_name(),
                    OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                ).unwrap()
            )
        }

        fn disconnect(&mut self) {
            *self.get_connection_mut() = None;
        }
    }

    #[derive(Default)]
struct DataBaseTest {
        connection: Option<Connection>,
    }
    impl OrmDataBase for DataBaseTest {
        fn get_connection(&self) -> &Option<Connection> {
            &self.connection
        }

        fn get_connection_mut(&mut self) -> &mut Option<Connection> {
            todo!()
        }

        fn get_name() -> String {
            todo!()
        }
    }
    impl DataBaseTest {
        fn connect(&mut self) {
            self.connection = Some(
                Connection::open_with_flags(
                    "DataBaseTest",
                    OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                    .unwrap(),
            );
        }
    }

}

pub mod attributes {

    pub trait SqlAttribute {
        fn to_query(self) -> String;
    }
    pub struct PrimaryKey;
    impl SqlAttribute for PrimaryKey {
        fn to_query(self) -> String {
            "PRIMARY KEY".to_string()
        }
    }

    struct Unique;

    impl SqlAttribute for Unique {
        fn to_query(self) -> String {
            "UNIQUE".to_string()
        }
    }

    pub struct AutoIncrement;

    impl SqlAttribute for AutoIncrement {
        fn to_query(self) -> String {
            "AUTO INCREMENT".to_string()
        }
    }

}


mod tests {
    struct WrapInt {
        val: i32,
    }

    impl From<i32> for WrapInt {
        fn from(value: i32) -> Self {
            WrapInt {val: value}
        }
    }

    impl Into<i32> for WrapInt {
        fn into(self) -> i32 {
            self.val
        }
    }

    use rusqlite::types::{FromSqlResult, ValueRef};
    use super::*;
    pub struct Users {
        name: String,
        id: WrapInt,
    }
    mod _name {
        use super::*;
        #[derive(Default)]
        pub struct name;

        impl TheType for name { type Type = String; }

        impl Into<RawTypes> for name {
            fn into(self) -> RawTypes {
                RawTypes::Column(RawColumn { table_name: "users".to_string(), name: "name".to_string() })
            }
        }

        impl Column for name {
            type Table = Users;
            const FULL_NAME: &'static str = "";

            fn get_name() -> String {
                "name".to_string()
            }
        }
    }
    use _name::name;

    mod _id {
        use super::*;

        pub struct id;

        impl Default for id {
            fn default() -> Self {
                id {}
            }
        }


        impl TheType for id { type Type = i32; }

        impl Into<RawTypes> for id {
            fn into(self) -> RawTypes {
                RawTypes::Column(RawColumn { table_name: "users".to_string(), name: "name".to_string() })
            }
        }

        impl Column for id {
            type Table = Users;
            const FULL_NAME: &'static str = "";

            fn get_name() -> String {
                "id".to_string()
            }
        }
    }

    use _id::id;
    use crate::attributes::SqlAttribute;

    impl Table for Users {
        fn get_name() -> String {
            "Users".to_string()
        }
    }

    impl Default for Users {
        fn default() -> Self {
            Users {
                name: "some_name".into(),
                id: (-1).into(),
            }
        }
    }

    impl FromSql for Users {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            todo!()
        }
    }

    impl OrmTable for Users {
        type ColumnsT = (<name as TheType>::Type, <id as TheType>::Type);
        fn columns(self) -> (<name as TheType>::Type, <id as TheType>::Type) {
            (self.name.into(), self.id.into())
        }

        fn columns_strings() -> Vec<OrmColumn> {
            let mut orm_column1: OrmColumn = id.into();
            orm_column1.attrs = vec![crate::attributes::PrimaryKey.to_query()];

            let mut orm_column2: OrmColumn = name.into();
            orm_column2.attrs = vec!["attrs".to_string()];
            vec![{ let mut column: OrmColumn = id.into(); column.attrs = vec![]; column }, orm_column2]
        }

        fn from_columns(columns: (<name as TheType>::Type, <id as TheType>::Type)) -> Self
        where
            Self: Sized,
        {
            Users {
                name: columns.0,
                id: columns.1.into(),
                ..Default::default()
            }
        }
    }

    #[test]
    fn test1 () {
        let create_q = Users::create_query();
        let insert_q = Users{ name: "bohdan".to_string(), id: 15.into() }.insert_query();

        println!("Create: {}" , create_q);
        println!("Insert: {:?}", insert_q);

        assert_eq!(Users{name: "hi".into(), id: 10.into()}.insert_query(), Users::from_columns(("hi".to_string(), 10)).insert_query());
    }
}
