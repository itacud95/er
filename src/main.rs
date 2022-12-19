use std::{process::exit, vec};

use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    complete();
    println!("in main")
}

struct Cmd {
    value: String,
    sub_commands: Vec<Cmd>,
}

impl Cmd {
    pub fn new(value: String, cmds: Vec<Cmd>) -> Self {
        Self {
            value: value,
            sub_commands: cmds,
        }
    }
    pub fn generate_completions(&self, current_arguments: &Vec<&str>) -> Vec<String> {
        for arg in current_arguments {
            for cmd in self.sub_commands.iter() {
                if arg == &cmd.value {
                    return cmd.generate_completions(current_arguments);
                }
            }
        }

        return self.generate_sub_commands();
    }
    pub fn generate_sub_commands(&self) -> Vec<String> {
        let mut commands = vec![];
        for name in self.sub_commands.iter() {
            //println!("{}::sub->{}", self.command, name.value());
            commands.push(name.value.clone());
        }
        return commands;
    }
}

fn complete() {
    let input = BashCompletionInput::from_env();

    match input {
        Ok(file) => write_out(&file),
        Err(_) => return,
    };
}

fn write_out(input: &BashCompletionInput) {
    let root = Cmd::new(
        String::from(""),
        vec![
            Cmd::new(
                String::from("build"),
                vec![
                    Cmd::new(String::from("sdk_basic_test"), vec![]),
                    Cmd::new(String::from("elf_test"), vec![]),
                    Cmd::new(String::from("obfuscated_ptr_test"), vec![]),
                ],
            ),
            Cmd::new(String::from("log"), vec![]),
            Cmd::new(String::from("checkout"), vec![]),
        ],
    );

    let current_args = input.args().clone();
    let output = root.generate_completions(&current_args);

    complete_string(input, output);
    exit(0)
}

fn complete_string(input: &BashCompletionInput, txt: Vec<String>) {
    let mut commands = vec![];
    for cmd in txt.iter() {
        if cmd.is_empty() {
            continue;
        }
        commands.push(cmd.as_str())
    }

    let completions = input.complete_subcommand(commands);
    completions.suggest();
}
