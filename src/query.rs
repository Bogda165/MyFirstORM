use std::marker::PhantomData;
use crate::column::Allowed;
use crate::safe_expressions::SafeExpr;

struct QueryPart<AllowedTables, ExprType> {
    tables: PhantomData<AllowedTables>,
    expr: SafeExpr<ExprType, AllowedTables>,
}



impl<AllowedTables, ExprType> QueryPart<AllowedTables, ExprType> {
    fn add_table<Table>(self) -> QueryPart<(Table, AllowedTables), ExprType> {
        QueryPart {
            tables: PhantomData::<(Table, AllowedTables)>,
            expr: SafeExpr::new(self.expr.expr),
        }
    }
}

mod tests {
    use std::marker::PhantomData;
    use my_macros::{from, table};
    use crate::create_a_name::Queryable;
    use crate::query::QueryPart;
    use crate::safe_expressions::SafeExpr;
    use crate::column::Allowed;
    use crate::column::Table;
    use crate::column::Column;
    use crate::literals::Bool;

    #[table]
    struct table1 {
        #[column]
        id: i32,
    }

    from!(table1);

    #[test]
    fn test1() {

        let part = QueryPart{ expr: QueryPart {expr: SafeExpr::basic(10), tables: PhantomData::<()> }.add_table::<table1>().expr.add(SafeExpr::<id, _>::column()), tables: PhantomData::<(table1, ())> };

        println!("{}", part.expr.expr.to_query());
    }
}