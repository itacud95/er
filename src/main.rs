use std::{process::exit, vec};

use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    complete();
    println!("in main")
}

struct CommandWrapper {
    commands: Vec<Cmd>,
}

impl CommandWrapper {
    pub fn new(commands: Vec<Cmd>) -> Self {
        Self { commands: commands }
    }
    pub fn generate_completions(&self, current_arguments: &Vec<&str>) -> Vec<String> {
        if current_arguments.len() < 3 {
            let mut output: Vec<String> = vec![];
            for cmd in &self.commands {
                output.push(String::from(&cmd.value))
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
            commands.push(name.value.clone());
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

fn get_completions(input: &impl CompletionInput) -> Vec<String> {
    let current_args = input.args().clone();

    let wrapper = CommandWrapper::new(vec![
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
    ]);

    return wrapper.generate_completions(&current_args);
}

fn generate_commands_and_exit(input: &BashCompletionInput) {
    let commands = get_completions(input);
    complete_string(input, commands);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_fetch() {
        let input = BashCompletionInput::from("er");
        let completions = get_completions(&input);
        assert_eq!(3, completions.len());
        assert_eq!("build", completions[0]);
        assert_eq!("log", completions[1]);
        assert_eq!("checkout", completions[2]);
    }

    #[test]
    fn complete_subcommand_build() {
        let input = BashCompletionInput::from("er build "); // todo: Why do we need space after build?
        let completions = get_completions(&input);
        assert_eq!(3, completions.len());
        assert_eq!("sdk_basic_test", completions[0]);
        // assert_eq!("log", completions[1]);
        // assert_eq!("checkout", completions[2]);
    }
}
