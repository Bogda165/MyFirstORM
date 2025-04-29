# Rust ORM for Sqlite

I recreated an orm in rust from scratch, because no one is hiring jnr rust devs ;(



SQLiteWrapper is a Rust project that provides an ORM (Object-Relational Mapping) layer for interacting with SQLite databases. It includes macros for defining tables, relationships, and queries.

## Features

- Define tables and relationships using macros.
- Generate SQL queries from Rust code.
- Support for one-to-one, one-to-many, and many-to-many relationships.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- SQLite

### Installation

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
rusqlite = "0.26.0"
quote = "1.0"
syn = "1.0"
proc-macro2 = "1.0"
```

### Usage

#### Defining Tables

Use the `table` macro to define a table:

```rust
#[derive(Default, Clone)]
#[table("users")]
struct Users {
    #[column]
    #[sql_type(Int)]
    #[constraint(PrimaryKey)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub text: String,
    #[relation(Address, address::table1)]
    #[relation_type = OneToMany]
    #[self_ident(users::id, Int)]
    pub addr: Address,
    pub some_val_that_wont_be_used_in_db: String,
}
```

#### Defining Relationships

Define relationships between tables using the `relation` attribute:

```rust
#[derive(Debug, Default, Clone)]
#[table("address")]
struct Address {
    #[column]
    #[sql_type(Int)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub _address: String,
}
```

#### Creating a Database

Use the `data_base` macro to define a database:

```rust
#[data_base(users::Users, address::Address)]
#[name = "Test_db"]
#[from(users::Users, address::Address)]
struct DataBaseTest {
    connection: Option<rusqlite::Connection>,
}
```

#### Running Queries

Generate and run queries using the provided macros:

```rust
fn main() {
    let users = Users::from_columns((10, "name".to_string()));
    let query = query_from!(users::Users)
        .join::<address::Address>(literal(10).less(column(address::id)))
        .select_test(((column(users::id), "val"), (column(address::id), "val2")));

    println!("Load query: {}", query.to_query());
}
```

## [DSL](dsl/README.md)
