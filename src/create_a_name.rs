use crate::column::Column;

/// Implement this trait with derive macro if you want to generate auto transformation into query. Read mor ein macro
pub trait AutoQueryable {
    fn to_query_auto(&self) -> Option<String> {
        None
    }

    fn get_query_auto() -> Option<String> {None}
}

/// Implement this create wiht derive masro, explisitly impl it, if you want to expand implementation
pub trait Queryable: AutoQueryable {
    fn convert_to_query(&self) -> Option<String>;

    fn to_query(&self) -> String {
        if let Some(query) = self.convert_to_query() {
            return query
        }
        if let Some(query) = Self::get_query_auto() {
            return query
        }
        self.to_query_auto()
            .expect("Nor impl with macro, explicit impl was provided")
    }

}

impl<T, U> AutoQueryable for (T, U)
where
    T: AutoQueryable,
    U: AutoQueryable,
{

}

impl<T, U> Queryable for (T, U)
where
    T: AutoQueryable + Queryable,
    U: AutoQueryable + Queryable,
{
    fn convert_to_query(&self) -> Option<String> {
        None
    }

    fn to_query(&self) -> String {
        format!("{}, {}", self.0.to_query(), self.1.to_query())
    }
}


mod tests {
    use super::*;

    impl AutoQueryable for u32 {}

    impl Queryable for u32 {
        fn convert_to_query(&self) -> Option<String> {
            Some("u32".to_string())
        }
    }

    impl AutoQueryable for u64 {}

    impl Queryable for u64 {
        fn convert_to_query(&self) -> Option<String> {
           Some("u64".to_string())
        }
    }

    #[test]
    fn test1() {
        println!("{}", u32::default().to_query());
        println!("{}", u64::default().to_query());
        println!("{}", (10 as u32, 15 as u64).to_query());

    }
}
