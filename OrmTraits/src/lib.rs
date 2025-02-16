use std::any::Any;
use dsl::column::{Column, RawColumn, Table};
use dsl::convertible::TheType;
use dsl::expressions::Expression::Raw;
use dsl::expressions::raw_types::RawTypes;
use crate::join::Join;

mod join {
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


struct OrmColumn {
    name: String,
    attrs: Vec<String>
}

impl Into<String> for OrmColumn{
    fn into(self) -> String {
        format!("({} {})", self.name, self.attrs.join(" "))
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
trait OrmTable<ColumnsT> : Table{
    fn columns(self) -> ColumnsT;
    /// Return a vec of all columns with their attributes, do not realise by yourself
    fn columns_strings() -> Vec<OrmColumn>;

    fn create_query() -> String {
        format!("CREATE TABLE {} ({})",
                Self::get_name(),
                Self::columns_strings().into_iter().join_iter(", ")

        )
    }

    fn insert_query(self) -> (String, ColumnsT)
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
}

mod tests {
    use super::*;
    pub struct Users {
        name: String,
        id: i32,
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

            fn get_name() -> String {
                "name".to_string()
            }
        }
    }
    use _name::name;

    mod _id {
        use super::*;
        #[derive(Default)]
        pub struct id;

        impl TheType for id { type Type = String; }

        impl Into<RawTypes> for id {
            fn into(self) -> RawTypes {
                RawTypes::Column(RawColumn { table_name: "users".to_string(), name: "name".to_string() })
            }
        }

        impl Column for id {
            type Table = Users;

            fn get_name() -> String {
                "id".to_string()
            }
        }
    }

    use _id::id;


    impl Table for Users {
        fn get_name() -> String {
            "Users".to_string()
        }
    }

    impl OrmTable<(String, i32)> for Users {
        fn columns(self) -> (String, i32) {
            (self.name, self.id)
        }

        fn columns_strings() -> Vec<OrmColumn> {
            let mut orm_column1: OrmColumn = id.into();
            orm_column1.attrs = vec!["attrs".to_string()];

            let mut orm_column2: OrmColumn = name.into();
            orm_column2.attrs = vec!["attrs".to_string()];
            vec![orm_column1, orm_column2]
        }
    }

    #[test]
    fn test1 () {
        let create_q = Users::create_query();
        let insert_q = Users{ name: "bohdan".to_string(), id: 15 }.insert_query();

        println!("Create: {}" , create_q);
        println!("Insert: {:?}", insert_q);
    }
}