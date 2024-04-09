use std::{
    fs::{self, File},
    io::Read,
};

use colored::Colorize;

pub mod adb;
pub mod cli;
pub mod config;
pub mod find;

fn read_config_file() -> i32 {
    let mut file = File::open("/home/jk/.pybuild").expect("failed to open config file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    println!("{}", content);
    return 0;
}

fn update_config_file(key: &str, value: &str) -> i32 {
    let file_path = "/home/jk/.pybuild";
    let mut output: Vec<String> = Vec::new();
    let mut found = false;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line.contains(key) {
            if found {
                // todo: find a better way.
                println!("error: found duplicates for key: {}", key);
                return -1;
            }
            found = true;
            let new_line = format!("{} = {}", key, value);
            output.push(new_line.to_string());
            continue;
        }
        output.push(line.to_string());
    }

    // write new config
    println!("new config: {:?}", output);
    fs::write(file_path, output.join("\n")).expect("Unable to write file");
    return 0;
}

fn create_options() -> Vec<cli::CommandOption> {
    use crate::adb::*;
    use crate::cli::create_category;
    use crate::cli::create_operation;
    vec![
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
        create_operation("find", find::find),
        create_category(
            "test",
            vec![
                create_operation("false", || return -1),
                create_operation("true", || return 1),
            ],
        ),
        create_category(
            "config",
            vec![create_category(
                "work",
                vec![create_category(
                    "arch",
                    vec![
                        create_operation("arm", || {
                            return update_config_file("android_abis", "arm64-v8a");
                        }),
                        create_operation("arm32", || {
                            return update_config_file("android_abis", "armeabi-v7a");
                        }),
                        create_operation("intel", || {
                            return update_config_file("android_abis", "x86_64");
                        }),
                        create_operation("intel32", || {
                            return update_config_file("android_abis", "x86");
                        }),
                        create_operation("32", || {
                            return update_config_file("android_abis", "x86,armeabi-v7a");
                        }),
                        create_operation("64", || {
                            return update_config_file("android_abis", "x86_64,arm64-v8a");
                        }),
                        create_operation("all", || {
                            return update_config_file(
                                "android_abis",
                                "x86_64,arm64-v8a,x86,armeabi-v7a",
                            );
                        }),
                    ],
                )],
            )],
        ),
        create_category("read", vec![create_operation("config", read_config_file)]),
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
