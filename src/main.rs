use colored::Colorize;

pub mod autocomplete;

/**Todo:
 *
 * - Income & outcome calculater
 *  - Save to local file
 *  - Save to firebase
 *
 */

fn test_function() -> i32 {
    println!("Test function!");
    return -1;
}

fn create_options() -> Vec<autocomplete::CommandOption> {
    use crate::autocomplete::create_operation;
    use crate::autocomplete::create_option;

    vec![
        // binaries
        create_option(
            "binaries",
            vec![
                create_option("show", vec![create_operation("test-file", test_function)]),
                create_option(
                    "write",
                    vec![create_operation("new-file.bin", test_function)],
                ),
            ],
        ),
        // gnu
        create_operation("gnu-plot", test_function),
        // test
        create_option(
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
