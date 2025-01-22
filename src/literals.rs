
///Time literal
pub struct Time {

}

///Date literal
pub struct Date {

}

/// Numbers literal change later to 64 instead of 32
pub enum Number {
    Real(f32),
    Int(i32),
}


//Bool literal
enum Bool {
    True,
    False
}

/// Literals
pub enum Lit {
    NumberLit(Number),
    StringLit(String),
    BlobLit,
    NULL,
    Bool(Bool),
    CurrentTime(Time),
    CurrentData(Date),
}