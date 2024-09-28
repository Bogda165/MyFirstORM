use MyTrait::MyTrait2;
use p_macros::table;
use Db_shit::*;

#[table("users")]
struct Users {
    #[INTEGER]
    #[AUTO_I]
    id: i32,
    #[TEXT]
    text: String,
    #[CONNECT("wpw")]
    wow: String
}

fn main() {
    let user = user{
        id: 10,
        name: "hello".to_string()
    };

    let loh = user.get_table();
    println!("{:?}", loh.name);

    let mut table = users::Users {
        id: 10,
        text: "My name is lOH".to_string(),
        wow: "loh".to_string()
    };

    table.id = 11;

    println!("{:?}", table.get_table())
}
