use colored::Colorize;

pub mod autocomplete;

fn test_function() -> i32 {
    println!("Test function!");
    return -1;
}

fn create_options() -> Vec<autocomplete::CommandOption> {
    use crate::autocomplete::create_category;
    use crate::autocomplete::create_operation;

    vec![
        // binaries
        create_category(
            "binaries",
            vec![
                create_category("show", vec![create_operation("test-file", test_function)]),
                create_category(
                    "write",
                    vec![create_operation("new-file.bin", test_function)],
                ),
            ],
        ),
        // gnu
        create_operation("gnu-plot", test_function),
        // test
        create_category(
            "test",
            vec![
                create_operation("false", || return -1),
                create_operation("true", || return 1),
            ],
        ),
    ]
}

fn main() {
    let operation = autocomplete::autocomplete(create_options());
    if operation.is_none() {
        // tab-completion
        return;
    }

    let operation = operation.unwrap();
    let ret_code = operation();
    if ret_code > 0 {
        let color = colored::Colorize::green("Success");
        println!("{}", color);
    } else {
        let msg = format!("Error: {}", ret_code).red();
        println!("{}", msg);
    }
}
