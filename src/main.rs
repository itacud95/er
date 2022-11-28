use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    let input = BashCompletionInput::from_env()
        .expect("Missing expected environment variables");

    let completions = input.complete_subcommand(vec!["add", "commit"]);

    completions.suggest();
}
