use crate::join::Join;
use dsl::column::{Column, RawColumn, Table};
use dsl::convertible::TheType;
use dsl::expressions::raw_types::RawTypes;
use dsl::expressions::Expression::Raw;
use rusqlite::types::{FromSql, Value};
use std::any::Any;
use std::fmt::Debug;
use rusqlite::{params, DatabaseName, Params, ToSql};
use crate::db::OrmDataBase;

pub mod join {
    pub trait Join
    where
        Self: Iterator,
    {
        fn join_iter(self, divide: &str) -> String;
    }

    impl<I: Iterator + Sized> Join for I
    where
        <I as Iterator>::Item: Into<String>,
    {
        fn join_iter(mut self, divide: &str) -> String {
            match self.next() {
                None => "".to_string(),
                Some(el) => self.fold(el.into(), |str1, str2| {
                    format!("{}{}{}", str1, divide, str2.into())
                }),
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
    pub attrs: Vec<String>,
}

impl Into<String> for OrmColumn {
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

#[derive(Debug, Clone)]
struct InsertQuerySignature {
    main_query: String,
    values_template: String,
    values_amount: usize,
}


impl InsertQuerySignature {

    fn new(main_query: String, values_template: String) -> Self {
        Self {
            main_query,
            values_template,
            values_amount: 1,
        }
    }
    fn generate_query(self) -> String {
        format!("{} VALUES ({});", self.main_query, self.values_template.repeat(self.values_amount))
    }
}

/// You must not implement this trait explicitly
pub trait OrmTable: Table + Default + FromSql {
    type ColumnsT;
    fn columns(self) -> Self::ColumnsT;
    /// Return a vec of all columns with their attributes, do not realise by yourself
    fn columns_strings() -> Vec<OrmColumn>;

    fn create_query() -> String {
        format!(
            "CREATE TABLE {} ({})",
            Self::get_name(),
            Self::columns_strings().into_iter().join_iter(", ")
        )
    }

    fn create<DataBase: OrmDataBase>(db: &DataBase) {
        let query = Self::create_query();

        db.query_post(&*query, params![]).unwrap()
    }

    fn insert_query() -> InsertQuerySignature
    where
        Self: Sized,
    {
        let query = format!(
            "INSERT INTO {0} ({1})",
            Self::get_name(),
            Self::columns_strings()
                .into_iter()
                .map(|column| { column.name })
                .join_iter(", "));
        let values = Self::columns_strings()
            .iter()
            .map(|_| { "?".to_string() })
            .join_iter(", ");
        InsertQuerySignature::new(query, values)
    }

    fn insert<DataBase: db::QueryableMarker>(self, db: &DataBase) where <Self as OrmTable>::ColumnsT: Params, <Self as OrmTable>::ColumnsT: Debug {
        let mut insert_query = Self::insert_query();
        insert_query.values_amount = 1;
        let columns = self.columns();

        db.query_post(&*insert_query.generate_query(), columns).unwrap();
    }

    ///Create a transaction inside itself
    fn insert_iterator<I: Iterator<Item = Self>, DataBase: db::OrmDataBase>(iterator: I, db: &mut DataBase) where <Self as OrmTable>::ColumnsT: Params, <Self as OrmTable>::ColumnsT: Debug {
        let tx = db.get_transaction().unwrap();

        iterator.for_each(|obj|{
            obj.insert(&tx);
        });

        tx.commit().unwrap();
    }

    fn from_columns(columns: Self::ColumnsT) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }
}

// pub mod repo {
//     use super::*;
//     use rusqlite::{Connection, Error, OpenFlags, Params};
//     pub trait OrmRepo<T: OrmTable>
//     where
//         Self: Default,
//     {
//         fn from_connection(connection: Connection) -> Self;
//         fn connect() -> Self
//         where
//             Self: Sized,
//         {
//             Self::from_connection(
//                 Connection::open_with_flags(
//                     T::get_name(),
//                     OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
//                 )
//                 .unwrap(),
//             )
//         }
//
//         fn get_connection(&self) -> &Option<Connection>;
//         fn create(&self) -> Result<(), Error> {
//             match self.get_connection() {
//                 None => Err(Error::InvalidQuery),
//                 Some(ref connection) => {
//                     let mut statement = match connection.prepare(&*T::create_query()) {
//                         Ok(stmt) => stmt,
//                         Err(error) => return Err(error),
//                     };
//                     if let Err(error) = statement.execute([]) {
//                         return Err(error);
//                     }
//                     Ok(())
//                 }
//             }
//         }
//         fn insert(&self, entity: T)
//         where
//             <T as OrmTable>::ColumnsT: Params,
//         {
//             let (q, v) = entity.insert_query();
//             match self.get_connection() {
//                 None => {
//                     panic!("table was not connected")
//                 }
//                 Some(connection) => {
//                     let mut statement = connection.prepare(&*q).unwrap();
//                     statement.execute(v).unwrap();
//                 }
//             }
//         }
//
//         // pub fn load(&mut self) {
//         //     let q = address::address::load("");
//         //     let mut statement = self.db_connection.prepare(&*q).unwrap();
//         //     statement
//         //         .query_map(
//         //             [],
//         //             |row: &Row| {
//         //                 let a_s = address::address::from_row(row);
//         //                 self.entities.push(address::Address::from_shadow_table(a_s));
//         //                 Ok(())
//         //             },
//         //         )
//         //         .unwrap()
//         //         .for_each(drop);
//         // }
//     }
// }

pub mod db {
    use dsl::convertible::Conversation;
    use dsl::query::the_query::Query;
    use rusqlite::ffi::Error;
    use rusqlite::{Connection, ErrorCode, OpenFlags, Params, Row, Rows, Statement, Transaction};
    use std::os::macos::raw::stat;
    use crate::OrmTable;

    pub trait QueryableMarker {
        fn query_get<F, T>(&self, query: &str, clos: F) -> Result<Vec<T>, rusqlite::Error>
        where
            F: FnMut(&Row) -> T;

        fn query_post<QParams: rusqlite::Params>(
            &self,
            query: &str,
            params: QParams,
        ) -> Result<(), rusqlite::Error>;

    }

    impl<DB: OrmDataBase> QueryableMarker for DB {
        fn query_get<F, T>(&self, query: &str, clos: F) -> Result<Vec<T>, rusqlite::Error>
        where
            F: FnMut(&Row) -> T
        {
            self.query_get(query, clos)
        }

        fn query_post<QParams: Params>(&self, query: &str, params: QParams) -> Result<(), rusqlite::Error> {
            self.query_post(query, params)
        }

    }

    impl QueryableMarker for Transaction<'_> {
        fn query_get<F, T>(&self, query: &str, mut clos: F) -> Result<Vec<T>, rusqlite::Error>
        where
            F: FnMut(&Row) -> T
        {
            let mut statement = self.prepare(query)?;
            let mut rows = statement.query([])?;
            let mut clos_result = vec![];

            while let Ok(row) = rows.next() {
                match row {
                    None => break,
                    Some(row) => {
                        clos_result.push(clos(row));
                    }
                }
            }

            Ok(clos_result)
        }

        fn query_post<QParams: Params>(&self, query: &str, params: QParams) -> Result<(), rusqlite::Error> {
            let mut statement = self.prepare(query)?;
            let _ = statement.execute(params)?;
            Ok(())
        }
    }

    pub trait OrmDataBase: Default {
        fn get_connection(&self) -> &Option<Connection>;

        fn get_connection_mut(&mut self) -> &mut Option<Connection>;

        fn query_get<F, T>(&self, query: &str, mut clos: F) -> Result<Vec<T>, rusqlite::Error>
        where
            F: FnMut(&Row) -> T,
        {
            match self.get_connection() {
                None => Err(rusqlite::Error::SqliteFailure(
                    Error::new(14),
                    Some("Probably the connection was not created".to_string()),
                )),
                Some(connection) => {
                    let mut statement = connection.prepare(query)?;
                    let mut rows = statement.query([])?;
                    let mut clos_result = vec![];

                    while let Ok(row) = rows.next() {
                        match row {
                            None => break,
                            Some(row) => {
                                clos_result.push(clos(row));
                            }
                        }
                    }

                    Ok(clos_result)
                }
            }
        }

        fn query_post<QParams: rusqlite::Params>(
            &self,
            query: &str,
            params: QParams,
        ) -> Result<(), rusqlite::Error> {
            match self.get_connection() {
                None => Err(rusqlite::Error::SqliteFailure(
                    Error::new(14),
                    Some("Probably the connection was not created".to_string()),
                )),
                Some(connection) => {
                    let mut statement = connection.prepare(query)?;
                    let _ = statement.execute(params)?;
                    Ok(())
                }
            }
        }

        fn get_name() -> String;

        fn connect(&mut self) {
            if self.get_connection().is_some() {
                return;
            }

            *self.get_connection_mut() = Some(
                Connection::open_with_flags(
                    Self::get_name(),
                    OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                    .unwrap(),
            )
        }

        fn get_transaction(&mut self) ->  Result<Transaction, rusqlite::Error> {
            match self.get_connection_mut() {
                None => Err(rusqlite::Error::SqliteFailure(
                    Error::new(14),
                    Some("Probably the connection was not created".to_string()),
                )),
                Some(ref mut connection) => {
                    connection.transaction()
                }
            }
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

pub mod static_from {
    pub trait StaticFrom<Type: ?Sized>
    where
        Self: Sized,
    {
        fn static_from() -> Self;
    }

    pub trait StaticInto<T>
    where
        T: StaticFrom<Self>
    {
        fn static_into() -> T {
            T::static_from()
        }
    }

    impl<T: StaticFrom<Self>> StaticInto<T> for T {}
}

pub mod relations {

    //#[relation(OneToOne, Address, connect_with = "address_id")] by default connect_with = format!("{}_{}", Address::table_name(), "id".into())
    //some_name: SomeTypeIntoAddress

    //#[relation(OneToMany, Address, connect_with = "user_id")] by default connect_with = format!("{}_{}", Address::table_name(), "id".into())
    //some_name: Vec<SomeTypeIntoAddress>

    use dsl::column::Table;

    pub mod relation_types {
        use super::*;
        use std::marker::PhantomData;
        use std::ops::Deref;
        use dsl::column::Column;
        use dsl::convertible::TheType;
        use p_macros::*;

        pub trait RelationType {}

        pub struct OneToOne;
        impl RelationType for OneToOne {}
        pub struct OneToMany;

        impl RelationType for OneToMany {}
        pub struct ManyToMany;

        impl RelationType for ManyToMany {}

        //
        pub trait HaveRelationWith<T: Table, RT>: Table + Sized
        {
            type RType: RelationType;
            type SelfIdent: Column<Table = Self, Type = RT>;
            type Ref: Column<Table = T, Type = RT>;
        }

        //TODO add check for pk or unique
        #[derive(Debug)]
        pub struct TableIdent<C: Column> {
            column: PhantomData<C>,
            table: PhantomData<<C as Column>::Table>,
            _type: <C as TheType>::Type,
        }

        #[derive(Debug)]
        pub struct RelationRecord<C1, C2>
        where
            C1: Column,
            C2: Column,
        {
            first_ident: TableIdent<C1>,
            second_ident: TableIdent<C2>,
        }

        #[derive(Default, Debug)]
        pub struct RelationStruct<T1: Table, T2: Table, TR1>
        where
            T1: HaveRelationWith<T2, TR1>,
        {
            relations: Vec<RelationRecord<<T1 as HaveRelationWith<T2, TR1>>::SelfIdent, <T1 as HaveRelationWith<T2, TR1>>::Ref>>
        }


        pub mod derefs_impl_for_relation_struct {
            use std::ops::DerefMut;
            use super::*;

            impl<T1, T2, TR1> Deref for RelationStruct<T1, T2, TR1>
            where
                T1: Table + HaveRelationWith<T2, TR1>, T2: dsl::column::Table
            {
                type Target = Vec<RelationRecord<<T1 as HaveRelationWith<T2, TR1>>::SelfIdent, <T1 as HaveRelationWith<T2, TR1>>::Ref>>;

                fn deref(&self) -> &Self::Target {
                    &self.relations
                }
            }

            impl<T1, T2, TR1> DerefMut for RelationStruct<T1, T2, TR1>
            where
                T1: Table + HaveRelationWith<T2, TR1>, T2: dsl::column::Table
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.relations
                }
            }
        }


        impl<T1, T2, TR1> RelationStruct<T1, T2, TR1>
        where
            T1: Table + HaveRelationWith<T2, TR1>,
            TR1: std::cmp::PartialEq + Clone, T2: dsl::column::Table
        {

            //consider all tables of type 1 have a relation with all tables of type2
            pub fn new(tables_type1: Option<Vec<T1>>, tables_type2: Option<Vec<T2>>) -> Self {
                let mut relations_records = vec![];
                if let Some(tables_type1) = tables_type1 {

                    tables_type1.iter().for_each(|table| {
                        //сheck if the value is the same -> add to the record
                        let first_key = <T1 as HaveRelationWith<T2, TR1>>::SelfIdent::get_value(table);

                        if let Some(ref tables_type2) = tables_type2 {

                            relations_records.extend(
                                tables_type2.iter().filter_map(|table| {
                                    let second_key = <T1 as HaveRelationWith<T2, TR1>>::Ref::get_value(table);

                                    if first_key == second_key {
                                        Some(RelationRecord {
                                            first_ident: TableIdent {
                                                column: PhantomData,
                                                table: PhantomData,
                                                _type: first_key.clone(),
                                            },
                                            second_ident: TableIdent {
                                                column: PhantomData,
                                                table: PhantomData,
                                                _type:  second_key,
                                            }
                                        })
                                    }else {
                                        None
                                    }
                                })
                            );

                        };
                    });

                }

                Self {
                    relations: relations_records,
                }
            }
        }
    }

    // use relation_types::*;
    //
    // struct Insert {
    //     main_table_insert: String,
    //     additional_table_insert: String,
    //     other_insert: String,
    // }
    //
    // trait RelationType {
    //     fn get_insert_prep<FromT: Table, ToT: Table>(ref_id: &str) -> Insert;
    // }
    //
    // impl RelationType for OneToOne {
    //     fn get_insert_prep<FromT: Table, ToT: Table>(ref_id: &str) -> Insert {
    //         Insert {
    //             main_table_insert: FromT::get_tale_insert_query(),
    //             additional_table_insert: ToT::get_tale_insert_query(),
    //             other_insert: "".into(),
    //         }
    //     }
    // }
    //
    // impl RelationType for OneToMany {
    //     fn get_insert_prep<FromT: Table, ToT: Table>(ref_id: &str) -> Insert {
    //         Insert {
    //             main_table_insert: FromT::get_tale_insert_query(),
    //             additional_table_insert: ToT::get_tale_insert_query(),
    //             other_insert: "".into(),
    //         }
    //     }
    // }
    //
    // impl RelationType for ManyToMany {
    //     fn get_insert_prep<FromT: Table, ToT: Table>(ref_id: &str) -> Insert {
    //         Insert {
    //             main_table_insert: FromT::get_tale_insert_query(),
    //             additional_table_insert: ToT::get_tale_insert_query(),
    //             other_insert: "".into(),
    //         }
    //     }
    // }
    //
    // trait Relation<T: Table>: Table {
    //     type RelationType: RelationType;
    //     const REF_ID_NAME: &'static str;
    //
    //     //fn get_relation() -> RelationStruct {}
    // }
}

mod tests {
    struct WrapInt {
        val: i32,
    }

    impl From<i32> for WrapInt {
        fn from(value: i32) -> Self {
            WrapInt { val: value }
        }
    }

    impl Into<i32> for WrapInt {
        fn into(self) -> i32 {
            self.val
        }
    }

    use super::*;
    use rusqlite::types::{FromSqlResult, ValueRef};
    pub struct Users {
        name: String,
        id: WrapInt,
    }
    mod _name {
        use super::*;
        #[derive(Default)]
        #[derive(Clone)]
        #[derive(Debug)]
        pub struct name;

        impl TheType for name {
            type Type = String;
        }

        impl Into<RawTypes> for name {
            fn into(self) -> RawTypes {
                RawTypes::Column(RawColumn {
                    table_name: "users".to_string(),
                    name: "name".to_string(),
                })
            }
        }

        impl Column for name {
            type Table = Users;
            const FULL_NAME: &'static str = "";

            fn get_name() -> String {
                "name".to_string()
            }

            fn get_value(table: &Self::Table) -> Self::Type {
                todo!()
            }
        }
    }
    use _name::name;

    mod _id {
        use super::*;

        #[derive(Clone)]
        #[derive(Debug)]
        pub struct id;

        impl Default for id {
            fn default() -> Self {
                id {}
            }
        }

        impl TheType for id {
            type Type = i32;
        }

        impl Into<RawTypes> for id {
            fn into(self) -> RawTypes {
                RawTypes::Column(RawColumn {
                    table_name: "users".to_string(),
                    name: "name".to_string(),
                })
            }
        }

        impl Column for id {
            type Table = Users;
            const FULL_NAME: &'static str = "";

            fn get_name() -> String {
                "id".to_string()
            }

            fn get_value(table: &Self::Table) -> Self::Type {
                todo!()
            }
        }
    }

    use crate::attributes::SqlAttribute;
    use _id::id;

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
            vec![
                {
                    let mut column: OrmColumn = id.into();
                    column.attrs = vec![];
                    column
                },
                orm_column2,
            ]
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
    fn test1() {
        let create_q = Users::create_query();
        let user_struct = Users {
            name: "bohdan".to_string(),
            id: 15.into(),
        };

        let insert_q = Users::insert_query();


        println!("Create: {}", create_q);
        println!("Insert: {:?}", insert_q);

    }
}
