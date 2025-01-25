use my_macros::{AutoQueryable, From, Queryable};
use crate::{Query, Queryable};
use crate::create_a_name::AutoQueryable;
use crate::expressions::Expression;

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
#[derive(Debug, Clone, Queryable, AutoQueryable)]
#[path = "crate::operators"]
pub enum NULLsExpression {
    IsNULL(Expression),
    IsNotNULL(Expression),
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


#[derive(Debug, AutoQueryable, Clone, Queryable)]
#[divide("AND,OR,XOR")]
#[path = "crate::operators"]
enum LogicalOperator {
    AND(Expression, Expression),
    OR(Expression, Expression),
    XOR(Expression, Expression),
}

// impl Queryable for LogicalOperator {
//     fn to_query(&self) -> String {
//         match self {
//             LogicalOperator::AND(expr1, expr2) => format!("({} AND {})", expr1.to_query(), expr2.to_query()),
//             LogicalOperator::OR(expr1, expr2) => format!("({} OR {})", expr1.to_query(), expr2.to_query()),
//             LogicalOperator::XOR(expr1, expr2) => format!("({} XOR {})", expr1.to_query(), expr2.to_query()),
//         }
//     }
// }

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
    Between(Expression, Expression, Expression)
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
    ExtractOperator(Expression, ExtractOperator),
}

impl Queryable for NonBinary {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            NonBinary::Collate(expr, _type) => {
                Some(format!("{} COLLATE {}", expr.to_query(), _type.to_query()))
            }
            NonBinary::ExtractOperator(_, _) => {
                None
            }
        }
    }
}

#[derive(Debug, Clone, AutoQueryable)]
#[path = "crate::operators"]
pub enum Operator {
    /// Only used for strings
    Concatenate(Expression, Expression, Expression),
    BinOperator(Binary),
    NonBinOperator(NonBinary),
}

impl Queryable for Operator {
    fn convert_to_query(&self) -> Option<String> {
        match self {
            Operator::Concatenate(expr1, divide, expr2) => {
                Some(format!("CONCAT({}, {}, {})", expr1.to_query(), divide.to_query(), expr2.to_query()))
            },
            _ => None,
        }
    }
}

mod tests {
    use crate::create_a_name::Queryable;
    use crate::expressions::Expression;
    use crate::expressions::Expression::Lit;
    use crate::literals::{Bool, Literal, Number};
    use crate::operators::{ArithmeticOperator, Binary, LogicalOperator, Operator};

    fn exclude_braces(mut query: String) -> String {
        query.replace("(", "").replace(")", "")
    }

    #[test]
    fn logical_operator() {
        let and_operator = Operator::BinOperator(
            Binary::LogicalOperator(
                LogicalOperator::AND(
                    Expression::Lit(Literal::Bool(Bool::True)),
                    Expression::Lit(Literal::Bool(Bool::False))
                )
            )
        );

        assert_eq!("True AND False", exclude_braces(and_operator.to_query()));
    }

    fn create_some_operators() -> crate::operators::Operator{
        let multiple = Operator::BinOperator(
            Binary::ArithmeticOperator(
                ArithmeticOperator::MUL(
                    Expression::Lit(Literal::NumberLit(Number::Int(10))),
                    Expression::Lit(Literal::NumberLit(Number::Int(15))),
                )
            )
        );

        Operator::BinOperator(
            Binary::ArithmeticOperator(
                ArithmeticOperator::ADD(
                    Expression::OperatorExpr(Box::new(multiple)),
                    Expression::Lit(Literal::NumberLit(Number::Int(18))),
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
}