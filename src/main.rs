use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};

/**
 * Todo: 
 * - Fix create default config
 * - Store config under ~/.confi
 */

use colored::Colorize;

pub mod autocomplete;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    apk: String,
}

fn write_to_json_file() -> File {
    let msg = format!("Generating default config").yellow();
    println!("{}", msg);
    let person = Config {
        apk: String::from("apk.apk"),
    };
    let json_string = serde_json::to_string(&person).expect("msg");

    match File::create("config.json") {
        Ok(mut file) => {
            file.write_all(json_string.as_bytes())
                .expect("Failed to write default config");
            return file;
        }
        Err(_) => {
            panic!("Failed to create default config")
        }
    };
}

fn read_config() -> Config {
    let mut file: File;
    match File::open("config.json") {
        Ok(f) => file = f,
        Err(_) => {
            file = write_to_json_file();
        }
    }

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("Failed to read file");

    // Deserialize the JSON string into your struct
    let person: Config = from_str(&json_string).expect("Failed to parse JSON");

    // Now you can work with the `person` struct
    println!("{:?}", person);

    return person;
}

fn install_apk() -> i32 {
    let config = read_config();

    println!("Installing apk!");
    let output = Command::new("adb")
        .args(&["install", "-t", config.apk.as_ref()])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("APK installed successfully");
        return 0;
    }
    println!("Failed to install APK");
    println!("Error: {:?}", output);
    return -1;
}

fn create_options() -> Vec<autocomplete::CommandOption> {
    use crate::autocomplete::create_category;
    use crate::autocomplete::create_operation;

    vec![
        // adb
        create_category("adb", vec![create_operation("install", install_apk)]),
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
