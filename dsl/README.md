# Rust SQL Query Builder

This project is a part of My ORM project, which you can find on my github.

Rust-based SQL query builder that allows for the creation and manipulation of SQL queries using Rust's type system and macros. The project includes various modules to handle different aspects of SQL query construction, such as literals, operators, and expressions.

## Features

- **AutoQueryable and Queryable Traits**: These traits are used to convert Rust types into SQL query strings. Macros are used to generate queries.
- **Expressions**: Support for various SQL expressions, including arithmetic, logical, comparison, and bitwise operations.
- **Literals**: Handling of different SQL literals like numbers, strings, booleans, dates, and times.
- **Operators**: Implementation of SQL operators such as LIKE, GLOB, REGEXP, MATCH, and more.
- **Safe Expressions**: Ensures type safety when constructing SQL queries.

## Modules

- `column`: Defines the `Column` struct and related functionality.
- `convertible`: Handles type conversions between different SQL types.
- `expressions`: Handles SQL expressions.
- `literals`: Defines various SQL literals.
- `operators`: Implements SQL operators.
- `queryable`: Contains the `AutoQueryable` and `Queryable` traits.
- `safe_expressions`: Ensures safe construction of SQL expressions.

## Compile-Time Type Checking
One of the key features of this Rust SQL Query Builder is that all type checking is performed at compile time. This ensures that any type mismatches or errors in SQL query construction are caught early, providing a safer and more reliable way to build SQL queries.

By leveraging Rust's powerful type system and compile-time checks, we can guarantee that:
- Only valid SQL expressions are constructed.
- Type safety is maintained throughout the query building process.
- Errors are caught during compilation, reducing runtime errors and improving code quality.

This approach provides a robust and efficient way to work with SQL queries in Rust, ensuring that your queries are both correct and safe.

## Example

Here is an example of how to use the Rust SQL Query Builder:

```rust
    let query = query_from!(users::users, address::address)
        .join::<phone::phone>(
            literal(10).less(column(phone::id))
                .and(
                    column(phone::number)
                        .cast::<i32>()
                        .div(literal(1000))
                        .equal(literal(9898))
                )
        ).where_clause(
            column(users::name)
                .like("%ll", Some(' '))
        ).select_test(
            (column(users::name),
                    (column(phone::number),
                     column(address::street)))
        );
```

This example demonstrates how to construct a SQL query using the provided macros and traits.
