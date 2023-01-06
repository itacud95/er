
fn private_function() {
    // function body goes here
    println!("private_function")
}

pub fn public_function() {
    // function body goes here
    private_function()
}

pub struct PublicStruct {
    // struct fields go here
}
