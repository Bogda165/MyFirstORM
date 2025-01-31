use my_macros::get_literal_type;


#[test]
fn test() {
    let x = 42;         // i32
    let y = 3.14;       // f64
    let z = "hello";    // &str
    let b = true;       // bool
    let s = String::from("world"); // String

    get_literal_type!(x); // Will print "Literal Type: i32 or u32"
    get_literal_type!(y); // Will print "Literal Type: f64"
    get_literal_type!(z); // Will print "Literal Type: String or &str"
    get_literal_type!(b); // Will print "Literal Type: bool"
    get_literal_type!(s); // Will print "Literal Type: String or &str"
}