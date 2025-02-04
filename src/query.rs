use std::marker::PhantomData;
use crate::column::Allowed;
use crate::convertible::TheType;
use crate::safe_expressions::SafeExpr;

struct QueryPart<AllowedTables, ExprType: TheType> {
    tables: PhantomData<AllowedTables>,
    expr: SafeExpr<ExprType, AllowedTables>,
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
}

// impl<AllowedTables, ExprType: TheType> Default for QueryPart<AllowedTables, ExprType> {
//     fn default() -> Self {
//         QueryPart {
//             tables: PhantomData::<()>,
//             expr: SafeExpr {
//                 tables: Default::default(),
//                 type_val: Default::default(),
//                 expr: Default::default(),
//             },
//         }
//     }
// }

mod tests {
    use std::marker::PhantomData;
    use my_macros::{from, table};
    use crate::create_a_name::Queryable;
    use crate::query::QueryPart;
    use crate::safe_expressions::SafeExpr;
    use crate::column::Allowed;
    use crate::column::Table;
    use crate::column::Column;
    use crate::convertible::TheType;
    use crate::expressions::RawTypes;
    use crate::literals::Bool;
    use crate::RawColumn;

    #[table]
    struct table1 {
        #[column]
        id: i32,
    }

    from!(table1);
    impl Default for id {
        fn default() -> Self {
            id {}
        }
    }

    impl Into<RawTypes> for id {
        fn into(self) -> RawTypes {
            RawTypes::Column(RawColumn{ table_name: "".to_string(), name: "id".to_string() })
        }
    }

    impl TheType for id {
        type Type = i32;
    }

    #[test]
    fn test1() {

        let part = QueryPart {expr: SafeExpr::basic(10), tables: PhantomData::<()> }.add_table::<table1>()
            .key_word(|expr| {
                expr.add(SafeExpr::<id, _>::column())
            });

        println!("{}", part.expr.expr.to_query());
    }
}