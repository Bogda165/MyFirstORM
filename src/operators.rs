use std::fmt::Debug;
use std::iter::Enumerate;
use std::marker::PhantomData;
use my_macros::{AutoQueryable, From, Queryable};
use crate::query::the_query::Query;
use crate::{Queryable};
use crate::queryable::AutoQueryable;
use crate::expressions::{Expression};
use crate::expressions::raw_types::RawTypes;
use crate::literals::{Bool, Literal, Null, Number};
use crate::safe_expressions::*;
use crate::convertible::*;
use crate::literals::Literal::StringLit;
use crate::operators::LGRM::GLOB;
use crate::operators::LikeExpression::LikeEscape;
use crate::operators::Operator::{BinOperator, NonBinOperator};

/// Collation need its own Expression https://www.sqlite.org/datatype3.html#collation
///
/// Return modified String
///
/// Example: Expression(Operator::Some_operator(Expression::OperatorExpression::Collate(Expression), Expression))
///
#[derive(Debug, Clone, AutoQueryable, Queryable)]
#[path = "crate::operators"]
enum CollateType {
    NOCASE,
    BINARY,
    UNICODE,
}

/// Casting is not safe, in case of type incompatibility will not result in compile time error
#[derive(Debug, Clone, AutoQueryable, Queryable)]
#[path = "crate::operators"]
enum CastType {
    TEXT,
    REAL,
    INTEGER,
    BLOB,
    NUMERIC,
}
impl From<String> for CastType {
    fn from(value: ( String )) -> Self { CastType::TEXT }
}
impl From<( f32 )> for CastType {
    fn from(value: ( f32 )) -> Self { Self::REAL}
}
impl From<( i32 )> for CastType {
    fn from(value: ( i32 )) -> Self { Self::INTEGER }
}
impl From<( Number )> for CastType {
    fn from(value: ( Number )) -> Self { Self::NUMERIC }
}



/// Extract Operator https://www.sqlite.org/json1.html#jptr
///
/// let In be -> and Into ->>
#[derive(Debug, Clone, AutoQueryable, Queryable)]
#[path = "crate::operators"]
enum ExtractOperator {
    In,
    Into
}

#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
pub enum NotExpression<T>
where T: Queryable
{
    NOT(T),
    Expr(T),
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn not(self) -> SafeExpr<T, AllowedTables>
    where T::Type: ConvertibleTo<Bool>
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: self.type_val,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(Binary::NOT(NotExpression::NOT(self.expr))))),
        }
    }
}

impl<T> From<T> for NotExpression<T>
where T: Queryable {
    fn from(value: T) -> Self {
        Self::Expr(value)
    }
}

impl<T> Queryable for NotExpression<T>
where T: Queryable
{
    fn convert_to_query(&self) -> Option<String> {
        match self {
            NotExpression::NOT(expr) => Some(format!("NOT {}", expr.to_query())),
            _ => None,
        }
    }
}

/// Working with NULLS
///
#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
pub enum NULLsExpression {
    ISNULL(Expression),
    ISNOTNULL(Expression),
}

impl Queryable for NULLsExpression {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            NULLsExpression::ISNULL(expr) => {Some(format!("(ISNULL {})", expr.to_query()))}
            NULLsExpression::ISNOTNULL(expr) => {Some(format!("(ISNOTNULL {})", expr.to_query()))}
        }
    }
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn is_null(self) -> SafeExpr<Bool, AllowedTables>
    where
        T: ConvertibleTo<Null>
    {
        SafeExpr::new(
            Expression::OperatorExpr(Box::new(Operator::BinOperator(NotExpression::Expr(NULLsExpression::ISNULL(self.expr)).into())))
        )
    }

    pub fn is_not_null(self) -> SafeExpr<Bool, AllowedTables>
    where
        T: ConvertibleTo<Null>
    {
        SafeExpr::new(
            Expression::OperatorExpr(Box::new(Operator::BinOperator(NotExpression::Expr(NULLsExpression::ISNOTNULL(self.expr)).into())))
        )

    }
}

///Like Expression
///
/// Not that second and Expression must be Lit::String, and the third must be a char
#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
enum LikeExpression {
    Like(Expression, Expression),
    LikeEscape(Expression, Expression, Expression)
}

impl Queryable for LikeExpression {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            LikeExpression::Like(expr, like) => Some(
                format!(
                    "({} LIKE {})",
                    expr.to_query(), like.to_query()
                )
            ),
            LikeExpression::LikeEscape(expr, like, escape) => Some(
                format!(
                    "({} LIKE {} ESCAPE {})",
                    expr.to_query(), like.to_query(), escape.to_query()
                )
            ),
        }
    }
}


/// LIKE, GLOB, REGEXP, MATCH
///
/// https://www.sqlite.org/lang_Expression.html#like
#[derive(Debug, Clone, AutoQueryable, Queryable)]
#[divide("path,GLOB,REGEX,MATCH")]
#[path = "crate::operators"]
enum LGRM{
    Like(LikeExpression),
    /// second Expression must be Lit::String
    GLOB(Expression, Expression),
    /// second Expression must be Lit::String
    REGEXP(Expression, Expression),
    /// I have now ideas what does it mean
    MATCH(Expression, Expression),
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn like(self, like_string: &str, escape: Option<char>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<String>
    {

        let like_string: Expression = Expression::Raw(Literal::StringLit(like_string.to_string()).into());

        let expr = match escape {
            None => {LikeExpression::Like(self.expr, like_string)}
            Some(escape) => {LikeEscape(self.expr, like_string, Expression::Raw(RawTypes::Lit(StringLit(escape.to_string()))))}
        };

        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(BinOperator(NotExpression::Expr(LGRM::Like(expr)).into()))),
        }
    }

    pub fn glob(self, like_string: &str, escape: Option<char>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<String>
    {

        let case_string: Expression = Expression::Raw(Literal::StringLit(like_string.to_string()).into());

        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(BinOperator(NotExpression::Expr(LGRM::GLOB(self.expr, case_string)).into()))),
        }
    }

    pub fn match_expr(self, like_string: &str, escape: Option<char>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<String>
    {

        let case_string: Expression = Expression::Raw(Literal::StringLit(like_string.to_string()).into());

        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(BinOperator(NotExpression::Expr(LGRM::MATCH(self.expr, case_string)).into()))),
        }
    }

    ///Later connect with some regex library
    pub fn regex(self, like_string: &str, escape: Option<char>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<String>
    {

        let case_string: Expression = Expression::Raw(Literal::StringLit(like_string.to_string()).into());

        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(BinOperator(NotExpression::Expr(LGRM::REGEXP(self.expr, case_string)).into()))),
        }
    }
}


#[derive(Debug, AutoQueryable, Clone, Queryable)]
#[divide("AND,OR,XOR")]
#[path = "crate::operators"]
enum LogicalOperator {
    AND(Expression, Expression),
    OR(Expression, Expression),
    XOR(Expression, Expression),
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn and<U: TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Bool>,
        U::Type: ConvertibleTo<Bool>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(LogicalOperator::AND(self.expr, expr.expr).into())))
        }
    }

    pub fn or<U: TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Bool>,
        U::Type: ConvertibleTo<Bool>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(LogicalOperator::OR(self.expr, expr.expr).into())))
        }
    }

    pub fn xor<U: TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Bool>,
        U::Type: ConvertibleTo<Bool>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(LogicalOperator::XOR(self.expr, expr.expr).into())))
        }
    }
}

#[derive(Debug, Queryable, Clone, AutoQueryable)]
#[divide("<=,<,>,>=,=")]
#[path = "crate::operators"]
enum ComparisonOperator {
    LessEqual(Expression, Expression),
    Less(Expression, Expression),
    More(Expression, Expression),
    MoreEqual(Expression, Expression),
    Equal(Expression, Expression),
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn less<U:TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Literal> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Literal> + Conversation<T::Type>,

    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ComparisonOperator::Less(self.expr, expr.expr).into())))
        }
    }

    pub fn more<U: TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Literal> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Literal> + Conversation<T::Type>,

    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ComparisonOperator::More(self.expr, expr.expr).into())))
        }
    }

    pub fn equal<U: TheType> (self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Bool, AllowedTables>
    where
        T::Type: ConvertibleTo<Literal> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Literal> + Conversation<T::Type>,

    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Bool>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ComparisonOperator::Equal(self.expr, expr.expr).into())))
        }
    }
}

/// only Binary operators, columns or numbers can be used
#[derive(Debug, Queryable, Clone, AutoQueryable)]
#[divide("+,-,*,/,%")]
#[path = "crate::operators"]
enum ArithmeticOperator {
    ADD(Expression, Expression),
    SUB(Expression, Expression),
    MUL(Expression, Expression),
    DIV(Expression, Expression),
    /// Only Binary operators, columns or integers
    MOD(Expression, Expression),
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn add<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<Number> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Number> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ArithmeticOperator::ADD(self.expr, expr.expr).into()))),
        }
    }

    pub fn sub<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<Number> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Number> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ArithmeticOperator::SUB(self.expr, expr.expr).into()))),
        }
    }

    pub fn mul<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<Number> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Number> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ArithmeticOperator::MUL(self.expr, expr.expr).into()))),
        }
    }

    pub fn div<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<Number> + Conversation<U::Type>,
        U::Type: ConvertibleTo<Number> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ArithmeticOperator::DIV(self.expr, expr.expr).into()))),
        }
    }

    pub fn module<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<i32> + Conversation<U::Type>,
        U::Type: ConvertibleTo<i32> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(ArithmeticOperator::ADD(self.expr, expr.expr).into()))),
        }
    }
}

/// Can be only used on integers and columns
#[derive(Debug, Queryable, Clone, AutoQueryable)]
#[divide("&,|,<<,>>")]
#[path = "crate::operators"]
enum BitwiseOperator {
    AND(Expression, Expression),
    OR(Expression, Expression),
    LeftShift(Expression, Expression),
    RightShift(Expression, Expression),
}

impl<T:TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn bit_and<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<i32> + Conversation<U::Type>,
        U::Type: ConvertibleTo<i32> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(BitwiseOperator::AND(self.expr, expr.expr).into()))),
        }
    }
    pub fn bit_or<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<i32> + Conversation<U::Type>,
        U::Type: ConvertibleTo<i32> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(BitwiseOperator::OR(self.expr, expr.expr).into()))),
        }
    }

    pub fn left_shift<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<i32> + Conversation<U::Type>,
        U::Type: ConvertibleTo<i32> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(BitwiseOperator::LeftShift(self.expr, expr.expr).into()))),
        }
    }
    pub fn right_shift<U:TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<Number, AllowedTables>
    where
        T::Type: ConvertibleTo<i32> + Conversation<U::Type>,
        U::Type: ConvertibleTo<i32> + Conversation<T::Type>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<Number>,
            expr: Expression::OperatorExpr(Box::new(Operator::BinOperator(BitwiseOperator::RightShift(self.expr, expr.expr).into()))),
        }
    }


}

/// All binary operators return Number
#[derive(Debug, AutoQueryable, Clone, From)]
#[path = "crate::operators"]
enum Binary {
    LogicalOperator(LogicalOperator),
    ComparisonOperator(ComparisonOperator),
    ArithmeticOperator(ArithmeticOperator),
    BitwiseOperator(BitwiseOperator),
    //Except(, ) later
    LGRM(NotExpression<LGRM>),
    NULLsExpression(NotExpression<NULLsExpression>),
    Between(Expression, Expression, Expression),
    NOT(NotExpression<Expression>),
}

impl Queryable for Binary {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            Binary::Between(expr1, between, and) =>
                Some(
                    format!("{} BETWEEN {} AND {}", expr1.to_query(), between.to_query(), and.to_query())
                ),
            _ => None
        }
    }
}

#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
enum NonBinary {
    Collate(Expression, CollateType),
    Cast(Expression, CastType),
    ExtractOperator(Expression, ExtractOperator),
}

impl Queryable for NonBinary {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            NonBinary::Collate(expr, _type) => {
                Some(format!("{} COLLATE {}", expr.to_query(), _type.to_query()))
            }
            NonBinary::Cast(expr, _type) => {
                Some(format!("CAST ({} AS {})", expr.to_query(), _type.to_query()))
            }
            NonBinary::ExtractOperator(_, _) => {
                None
            }
            _ => {
                None
            }
        }
    }
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn collate(self, ct: CollateType) -> SafeExpr<String, AllowedTables>
    where T::Type: ConvertibleTo<String>
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<String>,
            expr: Expression::OperatorExpr(Box::new(Operator::NonBinOperator(NonBinary::Collate(self.expr, ct)))),
        }
    }

    pub fn cast<U :TheType>(self) -> SafeExpr<U, AllowedTables>
    where U::Type: Into<CastType> + Default{
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<U>,
            expr: Expression::OperatorExpr(Box::new(NonBinOperator(NonBinary::Cast(self.expr, U::Type::default().into()))))
        }
    }
}

#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
pub enum Operator {
    /// Only used for strings
    Concatenate(Expression, Expression),
    BinOperator(Binary),
    NonBinOperator(NonBinary),
}

impl Queryable for Operator {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            Operator::Concatenate(expr1, expr2) => {
                Some(format!("({} || {})", expr1.to_query(), expr2.to_query()))
            },
            _ => None,
        }
    }
}

impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn concatenate<U: TheType>(self, expr: SafeExpr<U, AllowedTables>) -> SafeExpr<String, AllowedTables>
    where
        U::Type: ConvertibleTo<String>,
        T::Type: ConvertibleTo<String>,
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<String>,
            expr: Expression::OperatorExpr(Box::new(Operator::Concatenate(self.expr, expr.expr))),
        }
    }
}

mod tests {
    use crate::queryable::Queryable;
    use crate::expressions::{Expression};
    use crate::expressions::raw_types::RawTypes;
    use crate::literals::{Bool, Literal, Number};
    use crate::operators::{ArithmeticOperator, Binary, CastType, LogicalOperator, Operator};
    use crate::operators::NonBinary::Cast;
    use crate::operators::Operator::NonBinOperator;
    use crate::safe_expressions::SafeExpr;

    fn exclude_braces(mut query: String) -> String {
        query.replace("(", "").replace(")", "")
    }

    #[test]
    fn logical_operator() {
        let and_operator = Operator::BinOperator(
            Binary::LogicalOperator(
                LogicalOperator::AND(
                    Expression::Raw(Literal::Bool(Bool::True).into()),
                    Expression::Raw(Literal::Bool(Bool::False).into())
                )
            )
        );

        assert_eq!("True AND False", exclude_braces(and_operator.to_query()));
    }

    fn create_some_operators() -> crate::operators::Operator{
        let multiple = Operator::BinOperator(
            Binary::ArithmeticOperator(
                ArithmeticOperator::MUL(
                    Expression::Raw(Literal::NumberLit(Number::Int(10)).into()),
                    Expression::Raw(Literal::NumberLit(Number::Int(15)).into()),
                )
            )
        );

        Operator::BinOperator(
            Binary::ArithmeticOperator(
                ArithmeticOperator::ADD(
                    Expression::OperatorExpr(Box::new(multiple)),
                    Expression::Raw(Literal::NumberLit(Number::Int(18)).into()),
                )
            )
        )
    }
    #[test]
    fn arithmetic_operator() {

        let complex_operator = create_some_operators();

        println!("{}", complex_operator.clone().to_query());

        assert_eq!("10 * 15 + 18", exclude_braces(complex_operator.to_query()));
    }
    #[test]
    fn between() {
        let co = create_some_operators();

        let between_operator = Operator::BinOperator(Binary::Between(Box::new(co).into(), Literal::NumberLit(10.into()).into(), Literal::NumberLit(Number::Int(15)).into()));

        println!("{}", between_operator.clone().to_query());
        assert_eq!("10 * 15 + 18 BETWEEN 10 AND 15", exclude_braces(between_operator.to_query()));
    }

    #[test]
    fn comparison_operator() {
        let operator1: Expression = Literal::NumberLit(10.into()).into();
        assert_eq!("10", exclude_braces(operator1.to_query()))
    }

    #[test]
    fn cast_operator() {
        let operator = Expression::OperatorExpr(Box::new(NonBinOperator(Cast(Literal::NumberLit(10.into()).into(), CastType::INTEGER))));

        assert_eq!("CAST 10 AS INTEGER", exclude_braces(operator.to_query()));

        let safe_expression = SafeExpr::<i32, ()>::new(operator).cast::<String>().like("%like_this", None);
        //let wrong_safe_expression = SafeExpr::<i32, ()>::new(operator).like("%like_this", None);

        println!("{}", safe_expression.expr.to_query());

        //assert_eq!("CAST CAST 10 AS INTEGER AS TEXT", exclude_braces(safe_expression.expr.to_query()));
    }

    #[test]
    fn safe_expressions() {
        let lit: SafeExpr<_, ()> = SafeExpr::literal(Bool::True);
        let and_operator = lit.and(SafeExpr::literal(Bool::False));

        assert_eq!("True AND False", exclude_braces(and_operator.expr.to_query()));

        let less_operator: SafeExpr<_, ()> = SafeExpr::literal(10).less(SafeExpr::literal(10.15));

        assert_eq!("10 < 10.15", exclude_braces(less_operator.expr.to_query()));

        //let wrong_less_operator = SafeExpr::literal(10.124).less(SafeExpr::literal(Bool::True));

        let add_operator: SafeExpr<_, ()> = SafeExpr::literal(10).add(SafeExpr::literal(10));

        let wrong_add_operator: SafeExpr<_, ()> = SafeExpr::literal(10).add(SafeExpr::literal(Bool::True));

        //let mod_operat0r = SafeExpr::literal(10).module(SafeExpr::literal(10.25));

        assert_eq!("10 + 10", exclude_braces(add_operator.expr.to_query()));

        let like_expr: SafeExpr<_, ()> = SafeExpr::literal("hello_man".to_string()).like("%hello", None).not();

        println!("{}", like_expr.expr.to_query());

        assert_eq!("NOT \"hello_man\" LIKE \"%hello\"", exclude_braces(like_expr.expr.to_query()));
    }

}
