use colored::Colorize;

pub mod adb;
pub mod cli;
pub mod config;

fn create_options() -> Vec<cli::CommandOption> {
    use crate::adb::*;
    use crate::cli::create_category;
    use crate::cli::create_operation;
    vec![
        // adb
        create_category(
            "adb",
            vec![
                create_operation("install", install_apk),
                create_operation("logcat", adb_logcat),
                create_operation("file_log", || {
                    println!("implement me");
                    return -1;
                }),
            ],
        ),
        // linux utility
        create_operation("find", || {
            println!("implement me");
            return -1;
        }),
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
    let operation = cli::autocomplete(create_options());
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
