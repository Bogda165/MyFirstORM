# Rust SQL Query Builder

This project is a Rust-based SQL query builder that allows for the creation and manipulation of SQL queries using Rust's type system and macros. The project includes various modules to handle different aspects of SQL query construction, such as literals, operators, and expressions.

## Features

- **AutoQueryable and Queryable Traits**: These traits are used to convert Rust types into SQL query strings. Macros are used to generate queries.
- **Expressions**: Support for various SQL expressions, including arithmetic, logical, comparison, and bitwise operations.
- **Literals**: Handling of different SQL literals like numbers, strings, booleans, dates, and times.
- **Operators**: Implementation of SQL operators such as LIKE, GLOB, REGEXP, MATCH, and more.
- **Safe Expressions**: Ensures type safety when constructing SQL queries.

## Modules

- `column`: Defines the `Column` struct and related functionality.
- `create_a_name`: Contains the `AutoQueryable` and `Queryable` traits.
- `expressions`: Handles SQL expressions.
- `literals`: Defines various SQL literals.
- `operators`: Implements SQL operators.
- `play_types`: Additional types used in the project.
- `safe_expressions`: Ensures safe construction of SQL expressions.
- `convertible`: Handles type conversions between different SQL types.

## Usage

To use this project, include it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rust_sql_query_builder = { path = "path/to/your/project" }