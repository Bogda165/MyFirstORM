use std::io::LineWriter;
use std::marker::PhantomData;
use std::ops::Deref;
use crate::column::{Allowed, Column};
use crate::convertible::TheType;
use crate::query::from::FromTables;
use crate::query::join::Join;
use crate::safe_expressions::SafeExpr;

pub mod query_part {
    use std::marker::PhantomData;
    use crate::convertible::TheType;
    use crate::safe_expressions::SafeExpr;

    pub struct QueryPart<AllowedTables, ExprType: TheType> {
        tables: PhantomData<AllowedTables>,
        pub(crate) expr: SafeExpr<ExprType, AllowedTables>,
    }

    impl<ExprType: TheType, AllowedTables> From<SafeExpr<ExprType, AllowedTables>> for QueryPart<AllowedTables, ExprType> {
        fn from(value: SafeExpr<ExprType, AllowedTables>) -> Self {
            QueryPart {
                tables: PhantomData::<AllowedTables>,
                expr: value,
            }
        }
    }


    impl<ExprType: TheType, AllowedTables> SafeExpr<ExprType, AllowedTables> {
        pub fn add_table<Table>(self) -> QueryPart<(Table, AllowedTables), ExprType> {
            QueryPart {
                tables: PhantomData::<(Table, AllowedTables)>,
                expr: SafeExpr::new(self.expr),
            }
        }
    }


    impl<AllowedTables, ExprType: TheType> QueryPart<AllowedTables, ExprType> {
        pub fn add_table<Table>(self) -> QueryPart<(Table, AllowedTables), ExprType> {
            QueryPart {
                tables: PhantomData::<(Table, AllowedTables)>,
                expr: SafeExpr::new(self.expr.expr),
            }
        }
        pub fn key_word<TransformFunc, NewType: TheType>(self, tf: TransformFunc) -> QueryPart<AllowedTables, NewType>
        where
            TransformFunc: FnOnce(SafeExpr<ExprType, AllowedTables>) -> SafeExpr<NewType, AllowedTables>
        {
            QueryPart {
                tables: PhantomData::<AllowedTables>,
                expr: tf(self.expr)
            }
        }

        pub fn into_expr(self) -> SafeExpr<ExprType, AllowedTables> {
            self.expr
        }
    }

}

pub mod the_query {
    use std::marker::PhantomData;
    use crate::column::Table;
    use crate::convertible::TheType;
    use crate::queryable::{AutoQueryable, Queryable};
    use crate::literals::Bool;
    use crate::query::from::FromTables;
    use crate::query::join::Join;
    use crate::safe_expressions::{SafeExpr, SafeExprTuple};
    use crate::query::select::Select;

    pub struct Where<AllowedTables> {
        pub expr: SafeExpr<Bool, AllowedTables>,
    }

    impl<Tables> AutoQueryable for Where<Tables> {}

    impl<Tables> Queryable for Where<Tables> {
        fn convert_to_query(&self) -> Option<String> {
            Some(format!("WHERE {}\n", self.expr.to_query()))
        }
    }

    impl<AllowedTables> Where<AllowedTables> {
        pub fn add_expr(self, expr: SafeExpr<Bool, AllowedTables>) -> Self{
            Where {
                expr: self.expr.and(expr)
            }
        }
    }

    impl<AllowedTables> From<SafeExpr<Bool, AllowedTables>> for Where<AllowedTables> {
        fn from(value: SafeExpr<Bool, AllowedTables>) -> Self {
            Where {
                expr: value,
            }
        }
    }

    impl<AllowedTables> Default for Where<AllowedTables> {
        fn default() -> Self {
            Where {
                expr: SafeExpr::literal(Bool::True),
            }
        }
    }

    pub struct Query<AllowedTables> {
        pub from_clause: FromTables<AllowedTables>,
        pub where_clause: Where<AllowedTables>,
        pub joins: Vec<Join<AllowedTables>>,
        pub select: Select,
    }

    impl<Tables> AutoQueryable for Query<Tables> {}

    impl <Tables> Queryable for Query<Tables> {
        fn convert_to_query(&self) -> Option<String> {
            Some(format!{"{select_c}{from_c}{join_c}{where_c};",
                         where_c = self.where_clause.to_query(),

                         join_c = self.joins.iter()
                             .map(|join| join.to_query())
                             .collect::<Vec<String>>()
                             .join(""),

                         from_c = self.from_clause.to_query(),
                         select_c = self.select.to_query(),
            })
        }
    }

    impl<AllowedTables> Query<AllowedTables> {
        pub fn add_table<Table: crate::column::Table>(self) -> Query<(Table, (AllowedTables))> {
            Query {
                from_clause: self.from_clause.add_table::<Table>(),
                where_clause: self.where_clause.expr.change_tables::<(Table, (AllowedTables))>().into(),
                joins: self.joins.into_iter().map(|old_join| {
                    Join {
                        expr: old_join.expr.change_tables(),
                        table: old_join.table,
                    }
                }).collect::<Vec<Join<(Table, (AllowedTables))>>>(),
                select: self.select,
            }
        }

        pub fn where_clause(self, expr: SafeExpr<Bool, AllowedTables>) -> Self {
            Query {
                from_clause: self.from_clause,
                where_clause: self.where_clause.add_expr(expr),
                joins: self.joins,
                select: self.select,
            }
        }

        pub fn join<T: Table>(self, expr: SafeExpr<Bool, (T, (AllowedTables))>) -> Query<(T, (AllowedTables))> {
            let mut query = Query {
                from_clause: self.from_clause.type_change::<T>(),
                where_clause: self.where_clause.expr.change_tables::<(T, (AllowedTables))>().into(),
                joins: self.joins.into_iter().map(|old_join| {
                    Join {
                        expr: old_join.expr.change_tables(),
                        table: old_join.table,
                    }
                }).collect::<Vec<Join<(T, (AllowedTables))>>>(),
                select: self.select,
            };

            query.joins.push(Join {table: T::get_name(), expr});

            query
        }

        pub fn select<T: TheType>(mut self, expr: SafeExpr<T, AllowedTables>) -> Self {
            self.select = self.select.select(expr.expr, None);
            self
        }

        pub fn select_test<Safe: SafeExprTuple<AllowedTables> + Queryable>(mut self, columns: Safe) -> Self {
            self.select = self.select.select_test(columns);
            self
        }
    }

    impl<AllowedTables> Default for Query<AllowedTables> {
        fn default() -> Self {
            Query {
                from_clause: FromTables::default(),
                where_clause: Where { expr: SafeExpr::literal(Bool::True) },
                joins: vec![],
                select: Select::default(),
            }
        }
    }
}

pub mod from {
    use std::marker::PhantomData;
    use crate::query::the_query::Query;
    use crate::column::Table;
    use crate::queryable::{AutoQueryable, Queryable};

    pub struct FromTables<AllowedTables> {
        tables: PhantomData<AllowedTables>,
        tables_query: Option<String>
    }

    impl<AllowedTables> Default for FromTables<AllowedTables> {
        fn default() -> Self {
            FromTables {
                tables: PhantomData::<AllowedTables>,
                tables_query: None,
            }
        }
    }

    impl<AllowedTables> FromTables<AllowedTables> {
        pub fn add_table<T: Table>(self) -> FromTables<(T, AllowedTables)> {
            FromTables {
                tables: PhantomData::<(T, AllowedTables)>,
                tables_query: Some(format!("{}{}",
                                           match self.tables_query {
                                               None => { "".to_string() }
                                               Some(val) => { format!("{}, ", val) }
                                           }
                                           , &*T::get_name())),
            }
        }

        pub fn type_change<T: Table>(self) -> FromTables<(T, AllowedTables)> {
            FromTables {
                tables: PhantomData::<(T, AllowedTables)>,
                tables_query: self.tables_query,
            }
        }
    }

    impl<T> AutoQueryable for FromTables<T> {}

    impl<T> Queryable for FromTables<T> {
        fn convert_to_query(&self) -> Option<String> {
            Some(format!{"FROM {}\n", self.tables_query.clone().unwrap()})
        }
    }
    #[macro_export]
    macro_rules! nest_tuple {
        ($a:ty) => { ($a, ()) };

        ($a:ty, $($rest:ty),+) => {
            ($a, nest_tuple!($($rest),+))
        };
    }
    #[macro_export]
    macro_rules! from_tables {
        ($($a:ty),+) => {
            FromTables {tables: PhantomData::<nest_tuple!($($a),+)>}
        }
    }

    #[macro_export]
    ///creates a query from multiply tables
    macro_rules! query_from {
    ($($arg:ty),+) => {
        {
            let mut query = Query::<()>::default();
            $(
                let mut query = query.add_table::<$arg>();
            )+
            query
        }
    }
}}

pub mod select {
    use crate::queryable::{AutoQueryable, Queryable};
    use crate::expressions::Expression;
    use crate::type_check;

    #[derive(Default)]
    pub struct Select {
        query: String,
    }

    impl Select {
        pub fn select(self, expr: Expression, name: Option<&str>) -> Self {
            Select { query:
            self.query + & * expr.to_query() + &*match name {
                None => {"".to_string()}
                Some(name) => {format!(" AS {}", name)}
            } + "\n"
            }
        }

        pub fn select_test<Columns: Queryable>(self, columns: Columns) -> Self {
            Select{
                query: columns.to_query()
            }
        }
    }

    impl AutoQueryable for Select {}

    impl Queryable for Select {
        fn convert_to_query(&self) -> Option<String> {
            Some(format!{"SELECT {}\n", self.query})
        }
    }

    #[macro_export]
    macro_rules! select{
        ($($input:tt)*) => {
            {

                let result = some_select!($($input)*);;
                type_check!($($input)*);
                result
            }
        }
    }

    #[macro_export]
    macro_rules! some_select {
        () => {};
        ($ident:ty as $str:literal, $($rest:tt)*) => {
            ((SafeExpr::<$ident, _>::column(), $str), some_select!($($rest)*))
        };
        ($ident:ty, $($rest:tt)*) => {
            ((SafeExpr::<$ident, _>::column(), <$ident as Column>::FULL_NAME) , some_select!($($rest)*))
        };
        ($ident:ty as $str:literal) => {
            (SafeExpr::<$ident, _>::column(), $str)
        };
        ($ident:ty) => {
            (SafeExpr::<$ident, _>::column(), <$ident as Column>::FULL_NAME)
        };
    }
}

pub mod comp_time_names_check {
    use crate::column::Column;
    use crate::compare_macro;

    pub const fn get_constant<T: Column>() -> &'static str {
        T::FULL_NAME
    }

    pub const fn my_trait_check<Obj: Column>() {}
    pub const fn compare(str1: &'static str, str2: &'static str) -> bool {
        if str1.len() != str2.len() {
            return false;
        }

        let mut i = 0;

        while i < str1.len() {
            if str1.as_bytes()[i] != str2.as_bytes()[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    #[macro_export]
    macro_rules! get_str {
        ($val:literal) => {
            $val
        };

        ($val:ty) => {
            {
                my_trait_check::<$val>();
                get_constant::<$val>()
            }
        }
    }

    #[macro_export]
    macro_rules! compare_with_string {
        ($compare_string:literal, $_type:ty as $second_string:literal, $($rest:tt)*) => {
            if (compare($compare_string, $second_string)) {panic!("Equal names detected")}
            compare_with_string!($compare_string, $($rest)*)
        };

        ($compare_string:literal, $second_string:ty, $($rest:tt)*) => {
            if (compare($compare_string, get_str!($second_string))) {panic!("Equal names detected")}
            compare_with_string!($compare_string, $($rest)*)
        };

        ($_type:ty, $second_string:ty, $($rest:tt)*) => {
            if compare(get_constant::<$_type>(), get_str!($second_string)) {panic!("Equal names detected")}
            compare_with_string!($_type, $($rest)* )
        };

        ($_type:ty, $second_type:ty as $second_string:literal, $($rest:tt)*) => {
            if (compare(get_constant::<$_type>(), $second_string)) {panic!("Equal names detected")}
            compare_with_string!($_type, $($rest)* )
        };

        ($_type:ty, $second_type:ty as $end:literal) => {
            if (compare(get_constant::<$_type>(), $end)) {panic!("Equal names detected")}
        };

        ($_type:ty, $end:ty) => {
            if (compare(get_constant::<$_type>(), get_str!($end))) {panic!("Equal names detected")}
        };

        ($string:literal, $_type:ty as $end:literal) => {
            if (compare($string, $end)) {panic!("Equal names detected")}
        };

        ($string:literal, $end:ty) => {
            if (compare($string, get_str!($end))) {panic!("Equal names detected")}
        };
    }

    #[macro_export]
    macro_rules! compare_macro {
        ($end:ty) => {

        };

        ($end_t:ty as $end:literal) => {

        };

        ($_type:ty, $($rest:tt)*) => {
            compare_with_string!($_type, $($rest)*);
            compare_macro!($($rest)*);
        };

        ($_type:ty as $current_string:literal, $($rest:tt)*) => {
            compare_with_string!($current_string, $($rest)*);
            compare_macro!($($rest)*);
        }
    }

    #[macro_export]
    macro_rules! type_check {
        ($($input:tt)*) => {
            const _: () = {
                compare_macro!($($input)*);
            };
        }
    }

}

pub mod join {
    use std::marker::PhantomData;
    use crate::convertible::{ConvertibleTo, TheType};
    use crate::queryable::{AutoQueryable, Queryable};
    use crate::literals::Bool;
    use crate::query::from::FromTables;
    use crate::safe_expressions::SafeExpr;

    pub struct Join<AllowedTables> {
        pub expr: SafeExpr<Bool, AllowedTables>,
        pub table: String
    }

    impl<AllowedTables> AutoQueryable for Join<AllowedTables> {}

    impl<AllowedTables> Queryable for Join<AllowedTables>{
        fn convert_to_query(&self) -> Option<String> {
            Some(format!("JOIN {} ON {}\n", self.table, self.expr.to_query()))
        }
    }
}

mod tests {
    use my_macros::{from, table};
    use super::comp_time_names_check;
    use crate::type_check;
    use crate::compare_macro;
    use crate::compare_with_string;
    use crate::get_str;
    use crate::query::comp_time_names_check::get_constant;
    use crate::query::comp_time_names_check::my_trait_check;
    use crate::query::comp_time_names_check::compare;
    use crate::queryable::Queryable;
    use crate::safe_expressions::{literal, SafeExpr};
    use crate::column::Allowed;
    use crate::column::Table;
    use crate::column::Column;
    use crate::convertible::TheType;
    use crate::expressions::raw_types::RawTypes;
    use crate::{column, query_from, select, some_select};
    use crate::query::the_query::{Query, Where};
    use crate::column::RawColumn;

    mod tables {
        use super::*;
        pub mod table1 {
        use rusqlite::types::FromSqlResult;
        use rusqlite::types::ValueRef;
        use super::*;

            #[table]
            struct table1 {
                #[column]
                #[sql_type(Int)]
                id: i32,
                #[sql_type(Text)]
                #[column]
                name: String,
            }
        }

        pub mod table2 {
            use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;
use super::*;

            #[table]
            struct table2 {
                #[column]
                #[sql_type(Int)]
                id: f32,
                #[column]
                #[sql_type(Text)]
                adress: String,
            }
        }

        pub mod table3 {
            use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;
use super::*;

            #[table]
            struct table3 {
                #[column]
                #[sql_type(Int)]
                id: f32,
                #[column]
                #[sql_type(Text)]
                address: String,
            }
        }

        from!(table1::table1, table2::table2, table3::table3);
    }

    use tables::*;
    use crate::tables::{address, phone, users};

    #[test]
    fn where_test() {
        let mut wh = Where::<table1::table1>::default();
        wh = wh.add_expr(SafeExpr::<table1::id, _>::column().less(SafeExpr::literal(10)));

        println!("{}", wh.expr.expr.to_query());
    }

    #[test]
    fn query_test() {
        let id_check = SafeExpr::<table1::name, _>::column().like("some_string", None);
        let some_complex_check = SafeExpr::<table1::id, _>::column()
            .to_string()
            .concatenate(
                SafeExpr::<table2::id, _>::column().to_string()
            )
            .like(
                "%10%", Some('.')
            );

        let query = query_from!(table1::table1, table2::table2)
            .where_clause(
                SafeExpr::<table1::id, _>::column().less(SafeExpr::literal(10))
            ).where_clause(
            id_check
        ).where_clause(
            some_complex_check
        )
            ;

        println!("{}", query.where_clause.expr.expr.to_query());
    }

    mod test_beauty {
        use crate::query::query_part::QueryPart;
        use super::*;
        fn basic<Val>(val: Val) -> SafeExpr<Val, ()>
        where
            Val: Into<RawTypes> + TheType
        {
            SafeExpr::<Val, ()>::literal(val)
        }

        fn column<Column: TheType + column::Column, T: column::Allowed<<Column as column::Column>::Table>>() -> SafeExpr<Column, T> {
            SafeExpr::<Column, T>::column()
        }


        #[test]
        fn test1() {
            let part: QueryPart<_, _> =
                basic(10)
                    .add_table::<table1::table1>().into_expr()
                    .add(column::<table1::id, _>())
                    .to_string()
                    .concatenate(column::<table1::name, _>()).add_table::<table3::table3>().into_expr()
                    .equal(
                        column::<table3::id, _>().to_string()
                            .concatenate(column::<table3::address, _>())
                    )
                    .into();

            // let part: QueryPart<_, _> = SafeExpr::<_, ()>::literal(10).add_table::<table1>().into_expr()
            //     .add(SafeExpr::<id, _>::column())
            //     .to_string()
            //     .concatenate(SafeExpr::<name, _>::column()).into();

            println!("{}", part.expr.expr.to_query());
        }
    }
    #[test]
    fn test_joins() {
        let query = query_from!(table1::table1).
            where_clause(
                SafeExpr::literal(10)
                    .less(SafeExpr::<table1::id, _>::column())
            )
            .join::<table3::table3>(
                SafeExpr::<table1::id, _>::column()
                    .equal(
                        SafeExpr::<table3::id, _>::column()
                    )
            )
            .join::<table2::table2>(
                SafeExpr::<table1::id, _>::column()
                    .equal(
                        SafeExpr::<table2::id, _>::column()
                    )
            )
            .where_clause(
                SafeExpr::literal(15)
                    .mul(SafeExpr::literal(10))
                    .more(
                        SafeExpr::<table3::id, _>::column()
                    )
            )
            .select_test(((SafeExpr::<table1::id, _>::column(), "some_column"), ((SafeExpr::<table1::id, _>::column(), "some_another_column"), (SafeExpr::<table2::id, _>::column(), "way_another_column"))))
            .to_query();

        println!("{}", query);
    }


    #[test]
    fn select_macro_test() {
        let query = query_from!(users::users, address::address)
            .join::<phone::phone>(
                literal(10).less(column(phone::id))
                    .and(
                        column(phone::number)
                            .cast::<i32>()
                            .div(literal(1000))
                            .equal(literal(9898))
                    )
            )
        .select_test(select!(users::name as "name", phone::number as "number", address::street));

        println!("{}", query.to_query());
        assert_eq!(24242, 1);
    }

}