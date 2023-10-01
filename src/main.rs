use colored::Colorize;

pub mod adb;
pub mod autocomplete;
pub mod config;

fn create_options() -> Vec<autocomplete::CommandOption> {
    use crate::adb::*;
    use crate::autocomplete::create_category;
    use crate::autocomplete::create_operation;
    vec![
        // adb
        create_category(
            "adb",
            vec![
                create_operation("install", install_apk),
                create_operation("logcat", adb_logcat),
            ],
        ),
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
    if ret_code >= 0 {
        let color = colored::Colorize::green("Success");
        println!("{}", color);
    } else {
        let msg = format!("Error: {}", ret_code).red();
        println!("{}", msg);
    }
}
