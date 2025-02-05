use std::io::LineWriter;
use std::marker::PhantomData;
use std::ops::Deref;
use crate::column::Allowed;
use crate::convertible::TheType;
use crate::safe_expressions::SafeExpr;

struct QueryPart<AllowedTables, ExprType: TheType> {
    tables: PhantomData<AllowedTables>,
    expr: SafeExpr<ExprType, AllowedTables>,
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
    use crate::{column, RawColumn};

    mod table1 {
        use super::*;

        #[table]
        struct table1 {
            #[column]
            id: i32,
            #[column]
            name: String,
        }

    }

    mod table2 {
        use super::*;

        #[table]
        struct table2 {
            #[column]
            id: f32,
            #[column]
            address: String,
        }
    }

    mod table3 {
        use super::*;

        #[table]
        struct table3 {
            #[column]
            id: f32,
            #[column]
            address: String,
        }
    }

    from!(table1::table1, table2::table2, table3::table3);

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