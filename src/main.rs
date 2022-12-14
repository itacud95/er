use std::{process::exit, vec};
use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

use crate::xkeyboard::public_function;

pub mod xkeyboard;

fn main() {
    complete();
    println!("in main");
    public_function();
}

struct CommandWrapper {
    commands: Vec<Command>,
}

impl CommandWrapper {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
    pub fn generate_completions(&self, current_arguments: &Vec<&str>) -> Vec<&str> {
        if current_arguments.len() < 3 {
            let mut output: Vec<&str> = vec![];
            for cmd in &self.commands {
                output.push(&cmd.value)
            }
            return output;
        }

        for arg in current_arguments {
            for cmd in &self.commands {
                if &cmd.value == arg {
                    return cmd.generate_completions(current_arguments);
                }
            }
        }

        return vec![];
    }
}

struct Command {
    value: String,
    sub_commands: Vec<Command>,
}

impl Command {
    pub fn new(value: &str, cmds: Vec<Command>) -> Self {
        Self {
            value: value.to_string(),
            sub_commands: cmds,
        }
    }
    pub fn generate_completions(&self, current_arguments: &Vec<&str>) -> Vec<&str> {
        for arg in current_arguments {
            for cmd in self.sub_commands.iter() {
                if arg == &cmd.value {
                    return cmd.generate_completions(current_arguments);
                }
            }
        }
        return self.generate_sub_commands();
    }
    pub fn generate_sub_commands(&self) -> Vec<&str> {
        let mut commands: Vec<&str> = vec![];
        for name in self.sub_commands.iter() {
            commands.push(&name.value);
        }
        return commands;
    }
}

fn complete() {
    let input = BashCompletionInput::from_env();

    match input {
        Ok(file) => generate_commands_and_exit(&file),
        Err(_) => return,
    };
}

fn create_commands() -> CommandWrapper {
    return CommandWrapper::new(vec![
        Command::new(
            "build",
            vec![
                Command::new("sdk_basic_test", vec![]),
                Command::new("elf_test", vec![]),
                Command::new("obfuscated_ptr_test", vec![]),
            ],
        ),
        Command::new("log", vec![]),
        Command::new("checkout", vec![]),
        Command::new("xkeyboard", vec![]),
    ]);
}

fn generate_commands_and_exit(input: &BashCompletionInput) {
    let wrapper = create_commands();
    let commands = wrapper.generate_completions(&input.args());
    complete_string(input, commands);
    exit(0)
}

fn complete_string(input: &BashCompletionInput, txt: Vec<&str>) {
    let mut commands: Vec<&str> = vec![];
    for cmd in txt.iter() {
        if cmd.is_empty() {
            continue;
        }
        commands.push(cmd)
    }

    let completions = input.complete_subcommand(commands);
    completions.suggest();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_fetch() {
        let input = BashCompletionInput::from("er");
        let wrapper = create_commands();
        let completions = wrapper.generate_completions(&input.args());
        assert_eq!(4, completions.len());
        assert_eq!("build", completions[0]);
        assert_eq!("log", completions[1]);
        assert_eq!("checkout", completions[2]);
    }

    #[test]
    fn complete_subcommand_build() {
        let input = BashCompletionInput::from("er build "); // todo: Why do we need space after build?
        let wrapper = create_commands();
        let completions = wrapper.generate_completions(&input.args());
        assert_eq!(3, completions.len());
        assert_eq!("sdk_basic_test", completions[0]);
        // assert_eq!("log", completions[1]);
        // assert_eq!("checkout", completions[2]);
    }
}
