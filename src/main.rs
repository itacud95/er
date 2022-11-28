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
    let first_completions = vec!["add", "commit", "build", "test"];
    match input.arg_index() {
        1 => complete_string(input, first_completions),
        _ => complete_string(input, vec!["second"]),
    }
    exit(0)
}

fn complete_string(input: BashCompletionInput, txt: Vec<&str>) {
    let completions = input.complete_subcommand(txt);
    completions.suggest();
}
