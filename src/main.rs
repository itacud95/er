
use crate::xkeyboard::public_function;
use crate::autocomplete::autocomplete;

pub mod xkeyboard;
pub mod autocomplete;

// add a field to Data type: i32 name: foo
struct Data {
    msg: String,
    size: i32,
    foo: i32,
}

impl Data {
    fn new(msg: &str, size: i32, foo: i32) -> Self { 
        Self { msg: String::from(msg), size: (size), foo: (foo) }
     } 
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data: msg: {}, size: {}, foo: {}", self.msg, self.size, self.foo)
    }
}

fn foobar(mut foo: Data) -> Data {
    foo.msg = "not the same, but different".to_string();
    println!("foobar: {}", foo);
    return foo;
}

fn burrowing() {
    let foo = Data::new("msg", 230, 50);
    println!("{}", foo);

    let foo = foobar(foo);
    println!("{}", foo);
}

fn main() {
    autocomplete();
    println!("in main");
    public_function();
    burrowing();
}
