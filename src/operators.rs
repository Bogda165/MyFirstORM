use crate::{Expr, Query};

/// Collation need its own expression https://www.sqlite.org/datatype3.html#collation
///
/// Return modified String
///
/// Example: Expr(Operator::Some_operator(Expr::OperatorExpr::Collate(Expr), Expr))
///
enum CollateType {
    NonCase,
    Binary,
    UniCode,
}

/// Extract Operator https://www.sqlite.org/json1.html#jptr
///
/// let In be -> and Into ->>
enum ExtractOperator {
    In,
    Into
}

enum NotExpr<T> {
    NOT(T),
    Expr(T),
}

/// Working with NULLS
///

enum NULLsExpr {
    IsNULL(Expr),
    IsNotNULL(Expr),
}

///Like expression
///
/// Not that second and Expr must be Lit::String, and the third must be a char
enum LikeExpr {
    Like(Expr, Expr),
    LikeEscape(Expr, Expr, Expr)
}


/// LIKE, GLOB, REGEXP, MATCH
///
/// https://www.sqlite.org/lang_expr.html#like
enum LGRM{
    Like(LikeExpr),
    /// second expr must be Lit::String
    GLOB(Expr, Expr),
    /// second expr must be Lit::String
    REGEXP(Expr, Expr),
    /// I have now ideas what does it mean
    MATCH(Expr, Expr),
}

enum LogicalOperator {
    AND(Expr, Expr),
    OR(Expr, Expr),
    XOR(Expr, Expr),
}

enum ComparisonOperator {
    EqualAndOperator(SimpleComparisonOperator),
    Operator(SimpleComparisonOperator),
}

enum SimpleComparisonOperator {
    Less(Expr, Expr),
    More(Expr, Expr),
    Equal(Expr, Expr),
}

/// only Binary operators, columns or numbers can be used
enum ArithmeticOperator {
    ADD(Expr, Expr),
    SUB(Expr, Expr),
    MUL(Expr, Expr),
    DIV(Expr, Expr),
    /// Only Binary operators, columns or integers
    MOD(Expr, Expr),
}

/// Can be only used on integers and columns
enum BitwiseOperator {
    AND(Expr, Expr),
    OR(Expr, Expr),
    LeftShift(Expr, Expr),
    RightShift(Expr, Expr),
}

/// All binary operators return Number
enum Binary {
    LogicalOperator(LogicalOperator),
    ComparisonOperator(ComparisonOperator),
    ArithmeticOperator(ArithmeticOperator),
    BitwiseOperator(BitwiseOperator),
    //Except(, ) later
    LGRM(NotExpr<LGRM>),
    NULLsExpr(NotExpr<NULLsExpr>),
    Between(Expr, Expr, Expr)
}

enum NonBinary {
    Collate(Expr, CollateType),
    ExtractOperator(Expr, ExtractOperator),
}

pub enum Operator {
    /// Only used for strings
    Concatenate(Expr, Expr),
    BinOperator(Binary),
    NonBinOperator(NonBinary),
}