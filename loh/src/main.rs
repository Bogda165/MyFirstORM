
#[derive(Debug)]
struct Users {
    //INTEGER
    id: i32,
    #[TEXT]
    text: String
}

impl Users {
    pub fn new() -> Self {
        Users {
            id: 10,
            text: "hui".to_string()
        }
    }
}
fn main() {
    let mut users = Users::new();

    println!("{:?}", users);
}