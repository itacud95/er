use std::{
    fs::File, io::{BufRead, BufReader, Write}, process::{Command, Stdio}
};

use regex::Regex;

use crate::config::read_config;

pub fn install_apk() -> i32 {
    let config = read_config();

    println!("Installing apk!");
    let output = Command::new("adb")
        .args(&["install", "-t", config.apk.as_ref()])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("APK installed successfully");
        return adb_launch();
    }
    println!("Failed to install APK");
    println!("Error: {:?}", output);
    return -1;
}

pub fn adb_launch() -> i32 {
    let config = read_config();

    println!("Launching apk!");
    let output = Command::new("adb")
        .args(&[
            "shell",
            "am",
            "start",
            "-a",
            "android.intent.action.MAIN",
            "-n",
            config.apk_intent.as_ref(),
        ])
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

pub fn adb_logcat() -> i32 {
    let mut file = File::create("er_adb.log").expect("Unable to create file");

    Command::new("adb")
        .args(&["logcat", "-c"])
        .output()
        .expect("Failed to execute command");

    let mut adb_logcat = Command::new("adb")
        .args(&["logcat", "-v", "color"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("msg");

    // Create a buffered reader to read the command's output
    let stdout = adb_logcat.stdout.take().expect("Failed to open stdout");
    let reader = BufReader::new(stdout);

    // Continuously read and print the output
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
                let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let cleaned_text = re.replace_all(&line, "");
                _ = writeln!(file, "{}", cleaned_text);
            },
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    // Wait for the adb logcat command to finish (this will not be reached in this case)
    let status = adb_logcat.wait().expect("msg");
    if !status.success() {
        eprintln!("adb logcat command failed with exit code: {}", status);
    }
    return 0;
}
