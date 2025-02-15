use crate::literals::*;
use crate::operators::NULLsExpression;
use crate::column::RawColumn;

pub trait TheType {
    type Type;
}


/// if triat is implemented for T with <U> with Result = V, means that conversation between T and U is possible, and the result of it will be V
pub trait Conversation<Type>{
    type Result: Default;
}

/// for each new type implement Convertible to Null, and Converstaion with result Null
/// the implementation of ConvertileTo for T, with <U> means that T can be converted to U
pub trait ConvertibleTo<To> {}

#[macro_export]
macro_rules! convertible {
    ($from:ty, $to:ty) => {
        impl ConvertibleTo<$to> for $from {}
        impl Conversation<$to> for $from {type Result = $to; }
        impl Conversation<$from> for $to {type Result = $to; }
    };

    ($from:ty, $to:ty, $result:ty) => {
        impl ConvertibleTo<$to> for $from {}
        impl Conversation<$to> for $from {type Result = $result; }
        impl Conversation<$from> for $to {type Result = $result; }
    };
}



#[macro_export]
macro_rules! conversation {
    ($from:ty, $to:ty) => {
        impl Conversation<$to> for $from {type Result = $to; }
        impl Conversation<$from> for $to {type Result = $to; }
        impl ConvertibleTo<$to> for $from {}
        impl ConvertibleTo<$from> for $to {}
    };

    ($from:ty, $to:ty, $result:ty) => {
        impl Conversation<$to> for $from {type Result = $result; }
        impl Conversation<$from> for $to {type Result = $result; }
        convertible!($to, $result);
        convertible!($from, $result);
    };

    ($from:ty, $to:ty, $result1:ty, $result2:ty) => {
        impl Conversation<$to> for $from {type Result = $result1; }
        impl Conversation<$from> for $to {type Result = $result2; }
        impl ConvertibleTo<$to> for $from {}
        impl ConvertibleTo<$from> for $to {}
    };
}

#[macro_export]
macro_rules! self_converted {
    ($_type:ty) => {
        impl Conversation<$_type> for $_type {type Result = $_type;}
        impl ConvertibleTo<$_type> for $_type {}
        impl TheType for $_type {
            type Type = $_type;
        }
    };
}

self_converted!(Number);
self_converted!(String);
self_converted!(Bool);
self_converted!(Time);
self_converted!(Date);
self_converted!(RawColumn);
self_converted!(i32);
self_converted!(f32);
self_converted!(Literal);

conversation!(i32, f32, Number);
convertible!(Bool, i32);
convertible!(Date, String);
convertible!(Time, String);

convertible!(Number, Literal);
convertible!(String, Literal);
convertible!(Bool, Literal);
convertible!(Time, Literal);
convertible!(Date, Literal);
convertible!(RawColumn, Literal);
convertible!(i32, Literal);
convertible!(f32, Literal);

convertible!(Null, Number, Null );
convertible!(Null, String, Null);
convertible!(Null, Time, Null);
convertible!(Null, Date, Null);
convertible!(Null, RawColumn, Null);
convertible!(Null, i32, Null);
convertible!(Null, f32, Null);


conversation!(Bool, Number, Bool, Bool);


