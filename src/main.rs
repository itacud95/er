
use crate::xkeyboard::public_function;
use crate::autocomplete::autocomplete;

pub mod xkeyboard;
pub mod autocomplete;

fn main() {
    autocomplete();
    println!("in main");
    public_function();
    // public_function();
}
