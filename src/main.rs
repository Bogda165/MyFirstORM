mod expressions;
mod column;
mod literals;
mod operators;

struct RawColumn {
    table_name: String,
    name: String,
}
enum Expr {
    BitwiseExpr
}
enum Aggr {

}

enum Func {

}

enum Column {
    Raw(RawColumn),
    All,
    Expression(Expr),
    Aggregation(Aggr),
    Function(Func),
    Case,
    Query,
    Lit,
    NULL,
}
// for check
trait Query {
    fn select(self, columns: Vec<Column>) -> String {
        //creat a string query with given columns
    }
}

fn main() {
    println!("Hello, world!");
}
