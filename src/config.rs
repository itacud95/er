use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fs::File,
    io::{Read, Write},
    process::exit,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub apk: String,
    pub apk_intent: String,
}

pub fn config_location() -> String {
    // todo: on read once
    const CONFIG_LOCATION: &str = "/.er_config.json";
    match home::home_dir() {
        Some(home) => {
            let path = home.to_str().expect("msg").to_owned() + CONFIG_LOCATION;
            return path;
        }
        None => panic!("Failed to find home dir"),
    }
}

pub fn write_to_json_file() -> File {
    let msg = format!("Generating default config").yellow();
    println!("{}", msg);
    let config = Config {
        apk: String::from("apk.apk"),
        apk_intent: String::from("com.package.app/.MainActivity"),
    };
    let json_string = serde_json::to_string(&config).expect("msg");

    match File::create(config_location()) {
        Ok(mut file) => {
            println!(
                "{}{}",
                format!("Default config created: ").yellow(),
                config_location()
            );
            file.write_all(json_string.as_bytes())
                .expect("Failed to write default config");

            exit(1);
        }
        Err(_) => {
            panic!("Failed to create default config: {}", config_location())
        }
    };
}

pub fn read_config() -> Config {
    let mut file: File;
    match File::open(config_location()) {
        Ok(f) => file = f,
        Err(_) => {
            file = write_to_json_file();
        }
    }

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("Failed to read file");

    // Deserialize the JSON string into your struct
    let config: Config = from_str(&json_string).expect("Failed to parse JSON");

    // Now you can work with the `config` struct
    println!("{:?}", config);

    return config;
}
