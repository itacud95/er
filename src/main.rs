use std::process::exit;

use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    complete();
    println!("in main")
}

fn complete() {
    let input = BashCompletionInput::from_env();

    match input {
        Ok(file) => write_out(file),
        Err(_) => return,
    };
}

fn write_out(input: BashCompletionInput) {
    let completions = input.complete_subcommand(vec!["add", "commit", "build", "test"]);
    completions.suggest();
    exit(0)
}
