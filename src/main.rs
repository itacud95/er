use crate::autocomplete::autocomplete;

pub mod autocomplete;

fn main() {
    let operation = autocomplete();
    if operation.is_some(){
        // println!("is_some()");
        
        return;
        // operation.unwrap()();
    }
    else if operation.is_none() {
        // println!("is_none()");
    }
    // println!("in main");
    // operation();
    // println!("in main");
    // public_function();
    // burrowing();
}
