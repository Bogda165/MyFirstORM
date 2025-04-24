use std::{marker::PhantomData, num::NonZeroU8, vec::IntoIter};

use insert::{Insert, InsertData, InsertError};
use insert_query::InsertQuery;
use orm_traits::Entity;
use realtions::Relation;

/// I could use some strategy pattern here but I really love rust trait system so I will try to make it usable with nearly no runtime overhead)
trait Insertable: Clone + std::fmt::Debug {}

type Container<T> = Vec<T>;

type InsertResult = Result<InsertQuery, InsertError>;

pub mod insert_query {

    #[derive(Debug, Clone)]
    pub struct InsertQuery {
        pub query: String,
    }

    impl From<String> for InsertQuery {
        fn from(value: String) -> Self {
            InsertQuery {
                query: format!("Insert operation of {}", value),
            }
        }
    }
}

pub mod realtions {
    use crate::assosiation::Assosiation;
    use crate::basic_insert;
    use crate::insert::Insert;
    use crate::orm_traits::{Column, Entity, TheType};
    use crate::realated_table_insert::RelatedTableInsert;
    use crate::realated_table_insert::{self, RelatedTableInsertData};

    pub trait OrmRelation<T: Entity>: Entity + Clone + Relation<T>
    where
        Self::TargetKey: Column<Table = T>,
        Self::SourceKey: Column<Table = Self>,
    {
        type JoinTalbe: Entity + Clone;
        type UserInsert: Insert<Self>;

        fn get_join_table(&self) -> &Self::JoinTalbe;

        fn get_user_insert() -> Self::UserInsert;

        fn get_related_insert() -> Result<impl Insert<Self>, ()> {
            let realated_insert = RelatedTableInsert {
                data: RelatedTableInsertData {
                    insert: basic_insert::BasicInsert::<T, ()>::new(())?,
                    target_type: std::marker::PhantomData::<T>,
                    source_type: std::marker::PhantomData::<Self>,
                },
                inner: Self::get_user_insert(),
            };

            basic_insert::BasicInsert::new(realated_insert)
        }
    }

    pub trait Relation<T: Entity>: Entity + Clone
    where
        Self::TargetKey: Column<Table = T>,
        Self::SourceKey: Column<Table = Self>,
    {
        type TargetKey: TheType;
        type SourceKey: TheType;

        fn get_related<'a>(&'a self) -> impl Iterator<Item = &'a Assosiation<Self, T>> + Clone
        where
            T: 'a,
            Self: 'a,
            Self: Sized;

        fn get_related_iter<'a, I>(
            source_iter: I,
        ) -> impl Iterator<Item = &'a Assosiation<Self, T>> + Clone
        where
            T: 'a,
            Self: Sized + 'a,
            I: Iterator<Item = &'a Self> + Clone,
        {
            source_iter.flat_map(|source| source.get_related())
        }
    }
}

pub mod assosiation {
    use crate::orm_traits::{Column, Entity, TheType};
    use crate::realtions::Relation;

    #[derive(Debug, Clone)]
    pub enum Assosiation<S: Entity, E: Entity>
    where
        S: Relation<E>,
    {
        Kye(<<S as Relation<E>>::TargetKey as TheType>::Type),
        Instance(Box<E>),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use address::Address;
    use assosiation::Assosiation;
    use realtions::Relation;
    use user::User;

    pub mod user {
        use assosiation::Assosiation;
        use orm_traits::{Column, TheType};

        use super::*;

        #[derive(Debug, Clone)]
        pub struct User {
            pub id: i32,
            pub name: String,
            pub addr: Assosiation<User, Address>,
        }

        pub struct Id;

        impl TheType for Id {
            type Type = i32;
        }

        impl Column for Id {
            type Table = User;
        }

        pub struct Name;

        impl TheType for Name {
            type Type = String;
        }

        impl Column for Name {
            type Table = User;
        }

        impl Entity for User {
            fn columns_for_insert(&self) -> Vec<InsertQuery> {
                vec![self.id.to_string().into(), self.name.clone().into()]
            }
        }
    }

    pub mod address {
        use orm_traits::{Column, TheType};

        use super::*;

        #[derive(Debug, Clone)]
        pub struct Address {
            pub id: i32,
            pub title: String,
        }

        pub struct Id;

        impl TheType for Id {
            type Type = i32;
        }

        impl Column for Id {
            type Table = Address;
        }

        pub struct Title;

        impl TheType for Title {
            type Type = String;
        }

        impl Column for Title {
            type Table = Address;
        }

        impl Entity for Address {
            fn columns_for_insert(&self) -> Vec<InsertQuery> {
                vec![self.id.to_string().into(), self.title.clone().into()]
            }
        }
    }

    impl Relation<User> for Address {
        type TargetKey = user::Id;
        type SourceKey = address::Id;

        fn get_related<'a>(&'a self) -> impl Iterator<Item = &'a Assosiation<Self, User>> + Clone
        where
            User: 'a,
            Self: 'a,
            Self: Sized,
        {
            vec![].into_iter()
        }
    }

    impl Relation<Address> for User {
        type TargetKey = address::Id;
        type SourceKey = user::Id;

        fn get_related<'a>(&'a self) -> impl Iterator<Item = &'a Assosiation<Self, Address>> + Clone
        where
            Address: 'a,
            Self: 'a,
            Self: Sized,
        {
            vec![&self.addr].into_iter()
        }
    }
}

pub mod orm_traits {
    use crate::insert_query::InsertQuery;

    #[derive(Debug, Clone)]
    struct Users {
        id: i32,
        name: String,
    }

    impl Entity for Users {
        fn columns_for_insert(&self) -> Vec<InsertQuery> {
            vec![self.id.to_string().into(), self.name.clone().into()]
        }
    }

    pub trait Entity {
        fn columns_for_insert(&self) -> Vec<InsertQuery>;
    }

    impl Entity for () {
        fn columns_for_insert(&self) -> Vec<InsertQuery> {
            vec![]
        }
    }

    pub trait TheType {
        type Type: std::fmt::Debug + Clone;
    }

    pub trait Column: TheType {
        type Table: Entity;
    }
}

pub mod insert {
    #[derive(Debug, Clone)]
    pub enum InsertError {}

    impl std::fmt::Display for InsertError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Hui")?;
            Ok(())
        }
    }

    use orm_traits::Entity;

    use super::*;

    pub trait InsertData<T: Entity> {
        fn insert_entity(&mut self, entity: &T) -> Container<InsertResult>;
    }

    impl<T: Entity> InsertData<T> for () {
        fn insert_entity(&mut self, _: &T) -> Container<InsertResult> {
            vec![]
        }
    }

    /// Each Insert instance must implement Into (Self, Inner), so Self::get_inner(self_obj) will return None.
    pub trait Insert<T: Entity>: Default {
        type Inner: Insert<T>;
        type InsertData: InsertData<T>;

        fn split_mut(&mut self) -> (&mut Self::Inner, &mut Self::InsertData);

        fn insert<'a, I>(&'a mut self, entities: I) -> impl Iterator<Item = InsertResult>
        where
            T: 'a,
            Self: 'a,
            I: Iterator<Item = &'a T> + Clone,
        {
            let _entitites = entities.clone();

            let (inner, this) = self.split_mut();

            let inner = inner.insert(_entitites).into_iter();

            let iter = entities.map(|entity| this.insert_entity(entity)).flatten();

            inner.chain(iter)
        }
    }

    impl<T: Entity> Insert<T> for () {
        type Inner = ();

        type InsertData = ();

        /// Never use this method
        fn split_mut(&mut self) -> (&mut Self::Inner, &mut Self::InsertData) {
            unreachable!("You can not use this method")
        }

        fn insert<'a, I>(&'a mut self, _: I) -> impl Iterator<Item = InsertResult>
        where
            T: 'a,
            Self: 'a,
            I: Iterator<Item = &'a T> + Clone,
        {
            vec![].into_iter()
        }
    }
}

pub mod realated_table_insert {
    use super::*;

    pub struct RelatedTableInsertData<RI: Insert<E>, E: Entity, T: Entity> {
        pub insert: RI,
        pub target_type: PhantomData<E>,
        pub source_type: PhantomData<T>,
    }

    impl<RI: Insert<E>, E: Entity, T: Entity> InsertData<T> for RelatedTableInsertData<RI, E, T> {
        fn insert_entity(&mut self, entity: &T) -> Container<InsertResult> {
            unreachable!()
        }
    }

    pub struct RelatedTableInsert<T: Entity, I: Insert<T>, E: Entity, RI: Insert<E>>
    where
        T: Relation<E>,
    {
        pub inner: I,
        pub data: RelatedTableInsertData<RI, E, T>,
    }

    impl<T: Entity, I: Insert<T>, E: Entity, RI: Insert<E>> Default for RelatedTableInsert<T, I, E, RI>
    where
        T: Relation<E>,
    {
        fn default() -> Self {
            RelatedTableInsert {
                inner: I::default(),
                data: RelatedTableInsertData {
                    insert: RI::default(),
                    target_type: PhantomData::<E>,
                    source_type: PhantomData::<T>,
                },
            }
        }
    }

    impl<T: Entity, I: Insert<T>, E: Entity, RI: Insert<E>> Insert<T>
        for RelatedTableInsert<T, I, E, RI>
    where
        T: Relation<E>,
    {
        type Inner = I;
        type InsertData = RelatedTableInsertData<RI, E, T>;

        fn split_mut(&mut self) -> (&mut Self::Inner, &mut Self::InsertData) {
            (&mut self.inner, &mut self.data)
        }

        fn insert<'a, Iter>(&'a mut self, entities: Iter) -> impl Iterator<Item = InsertResult>
        where
            T: 'a,
            Self: 'a,
            Iter: Iterator<Item = &'a T> + Clone,
        {
            let _entitites = entities.clone();

            let (inner, this): (&mut Self::Inner, &mut Self::InsertData) = self.split_mut();

            let inner = inner.insert(_entitites).into_iter();

            let iter = <T as Relation<E>>::get_related_iter(entities).filter_map(|assosiation| {
                match assosiation {
                    assosiation::Assosiation::Kye(_) => None,
                    assosiation::Assosiation::Instance(obj) => Some(obj.as_ref()),
                }
            });

            let iter = this.insert.insert(iter);

            inner.chain(iter)
        }
    }

    impl<T: Entity, I: Insert<T>, E: Entity, RI: Insert<E>> RelatedTableInsert<T, I, E, RI>
    where
        T: Relation<E>,
    {
        pub fn new(insert: I) -> Result<Self, ()> {
            Ok(RelatedTableInsert {
                inner: insert,
                data: RelatedTableInsertData {
                    insert: RI::default(),
                    target_type: PhantomData::<E>,
                    source_type: PhantomData::<T>,
                },
            })
        }
    }
}

pub mod basic_insert {
    use super::*;

    pub struct BasicInsertData<T: Entity> {
        _maker: PhantomData<T>,
    }

    impl<T: Entity> InsertData<T> for BasicInsertData<T> {
        fn insert_entity(&mut self, entity: &T) -> Container<InsertResult> {
            entity
                .columns_for_insert()
                .into_iter()
                .map(|res| Ok(res))
                .collect()
        }
    }
    pub struct BasicInsert<T: Entity, I: Insert<T>> {
        pub inner: I,
        pub data: BasicInsertData<T>,
    }

    impl<T: Entity, I: Insert<T>> Default for BasicInsert<T, I> {
        fn default() -> Self {
            Self::new(I::default()).unwrap()
        }
    }

    impl<T: Entity, I: Insert<T>> Insert<T> for BasicInsert<T, I> {
        type Inner = I;

        type InsertData = BasicInsertData<T>;

        fn split_mut(&mut self) -> (&mut Self::Inner, &mut Self::InsertData) {
            (&mut self.inner, &mut self.data)
        }
    }

    impl<T: Entity, I: Insert<T>> BasicInsert<T, I> {
        pub fn new(insert: I) -> Result<Self, ()> {
            Ok(BasicInsert {
                inner: insert,
                data: BasicInsertData {
                    _maker: PhantomData::<T>,
                },
            })
        }
    }
}

#[cfg(test)]
pub mod tests2 {
    use tests::{address::Address, user::User};

    use super::*;

    #[test]
    fn test1() -> Result<(), ()> {
        let basic_insert = basic_insert::BasicInsert::<User, _>::new(())?;

        let realated_insert = realated_table_insert::RelatedTableInsert::<
            User,
            _,
            Address,
            basic_insert::BasicInsert<Address, ()>,
        >::new(basic_insert);

        Ok(())
    }
}
