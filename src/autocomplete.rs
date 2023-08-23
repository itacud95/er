use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};
use std::{process::exit, vec};

pub fn autocomplete() -> Option<fn() -> i32>{
    // complete();

    let input = match BashCompletionInput::from_env() {
        Err(_) => {
            println!("not completing");
            //todo: run_autocomplete() but only return the operation - not completing. 
            return None
        },
        Ok(input) => return run_autocomplete(&input),
    };
}

fn create_strings_from_vector(options: &Vec<CommandOption>) -> Vec<&str> {
    let mut strings: Vec<&str> = Vec::new();
    for option in options {
        strings.push(option.readable.as_str());
    }
    return strings;
}

fn get_current_option(input: Vec<&str>, options: &Vec<CommandOption>) -> Option<CommandOption> {
    let mut current_option= None;
    let mut current_list = options;
    for typed in &input {
        for option in current_list {
            if typed == &option.readable {
                current_option = Some(option);
                if let OptionType::Options(suboptions) = &current_option.unwrap().option_type{
                    // more to lookup, continue
                    current_list = suboptions;
                } 
                else if let OptionType::Operation(_operation) = &current_option.unwrap().option_type {
                    //this is the last command
                    return current_option.cloned();
                }
                // return Some(option.clone());
            }
        }
    }

    // if not found, return root
    // create_strings_from_vector(&options);

    return current_option.cloned();
}

fn run_autocomplete(input: &BashCompletionInput) -> Option<fn() -> i32> {

    let options = initialize_options();
    let current_option = get_current_option(input.args(), &options);
    
    if current_option.is_some() {
        let current_option = current_option.unwrap();
        if let OptionType::Options(suboptions) = &current_option.option_type {
            let autocomplete_options = create_strings_from_vector(suboptions);
            let completions = input.complete_subcommand(autocomplete_options);
            completions.suggest();
            // exit(0);
        }
        else if let OptionType::Operation(operation) = &current_option.option_type {
            return Some(*operation);
        }
    }else if current_option.is_none() { 
        let autocomplete_options = create_strings_from_vector(&options);
        let completions = input.complete_subcommand(autocomplete_options);
            completions.suggest();
            // exit(0);
    }
    return None;

}

fn test_function() -> i32 {
    println!("Test function!");
    return -1;
}

#[derive(Clone)]
enum OptionType {
    Options(Vec<CommandOption>),
    Operation(fn() -> i32),
}

#[derive(Clone)]
struct CommandOption {
    readable: String,
    option_type: OptionType,
}

fn create_option(readable: &str, options: Vec<CommandOption>) -> CommandOption {
    CommandOption {
        readable: readable.to_string(),
        option_type: OptionType::Options(options),
    }
}

fn create_operation(readable: &str, operation: fn() -> i32) -> CommandOption {
    CommandOption {
        readable: readable.to_string(),
        option_type: OptionType::Operation(operation),
    }
}

fn testit() {
    let option = CommandOption {
        readable: "readable".to_string(),
        option_type: OptionType::Options(vec![]),
    };
    let operation = CommandOption {
        readable: "readable".to_string(),
        option_type: OptionType::Operation(test_function),
    };
}

fn initialize_options() -> Vec<CommandOption> {
    vec![
        create_option(
            "do_some",
            vec![
                create_option(
                    "other_stuff",
                    vec![create_operation("final_thing", test_function)],
                ),
                create_option("or_this", vec![create_operation("one", test_function)]),
            ],
        ),
        create_operation("do_it_right_away", test_function),
    ]
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
        Command::new("test", vec![]),
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
    fn complete_subcommand_build() {
        let input = BashCompletionInput::from("er build "); // todo: Why do we need space after build?
        let wrapper = create_commands();
        let completions = wrapper.generate_completions(&input.args());
        assert_eq!(3, completions.len());
        assert_eq!("sdk_basic_test", completions[0]);
        // assert_eq!("log", completions[1]);
        // assert_eq!("checkout", completions[2]);
    }

    #[test]
    fn test_options() {
        let options = initialize_options();
        let arguments = vec!["er "];
        let current_option = get_current_option(arguments, &options);
        
        assert!(current_option.is_none());
        
        let autocomplete_options = create_strings_from_vector(&options);
        assert!(autocomplete_options.len() > 0);
        assert!(autocomplete_options.contains(&"do_some".to_string().as_str()));
    }

    #[test]
    fn test_options_longer() {
        let options = initialize_options();
        let arguments = vec!["er", "do_some", "or_this"];
        let current_option = get_current_option(arguments, &options);
        
        assert!(current_option.is_some());
        
        if current_option.is_some() {
            let current_option = current_option.unwrap();
            if let OptionType::Options(suboptions) = &current_option.option_type {
                let autocomplete_options = create_strings_from_vector(suboptions);
            }
            else if let OptionType::Operation(operation) = &current_option.option_type {
                let func = operation;
            }
        }
        
        // let autocomplete_options = create_strings_from_vector(&options);
        // assert!(autocomplete_options.len() > 2);
    }
}
