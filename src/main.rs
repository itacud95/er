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
        create_operation("gnu-plot", test_function),
    ]
}

fn main() {
    let operation = autocomplete::autocomplete(create_options());
    if operation.is_none() {
        // tab-completion
        return;
    }

    let operation = operation.unwrap();
    operation();
}
