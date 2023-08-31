use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

extern crate colored;
use colored::*;

#[derive(Clone)]
enum OptionType {
    Options(Vec<CommandOption>),
    Operation(fn() -> i32),
}

#[derive(Clone)]
pub struct CommandOption {
    readable: String,
    option_type: OptionType,
}

pub fn create_option(readable: &str, options: Vec<CommandOption>) -> CommandOption {
    CommandOption {
        readable: readable.to_string(),
        option_type: OptionType::Options(options),
    }
}

pub fn create_operation(readable: &str, operation: fn() -> i32) -> CommandOption {
    CommandOption {
        readable: readable.to_string(),
        option_type: OptionType::Operation(operation),
    }
}

pub fn autocomplete(options: Vec<CommandOption>) -> Option<fn() -> i32> {
    let autocompleter = Autocomleter { options: options };

    match BashCompletionInput::from_env() {
        Err(_) => {
            let args: Vec<String> = std::env::args().collect();
            let v8: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
            let current_option = autocompleter.get_current_option(v8);

            if current_option.is_some() {
                if let OptionType::Operation(operation) = &current_option.unwrap().option_type {
                    return Some(operation.to_owned());
                }
            }
            let args = args.join(" ").yellow();
            println!("'{}' is not a valid command", args);
            for help in autocompleter.get_help() {
                println!("{}", help);
            }
        }
        Ok(input) => {
            let completions = autocompleter.tab_complete(input);
            completions.suggest();
            return None;
        }
    };

    return None;
}

trait AutocomleteOperions {
    fn get_help(&self) ->Vec<String>;
    fn tab_complete(&self, input: BashCompletionInput) -> Vec<String>;
    fn get_current_option(&self, input: Vec<&str>) -> Option<CommandOption>;
}

struct Autocomleter {
    options: Vec<CommandOption>,
}

impl AutocomleteOperions for Autocomleter {
    fn get_help(&self) -> Vec<String> {
        let mut msg: Vec<String> = vec![];
        msg.push(String::from("Options: "));
        for opt in &self.options {
            if let OptionType::Options(_operation) = &opt.option_type {
                msg.push(format!("\t{} ->", opt.readable));
            } else {
                msg.push(format!("\t{}", opt.readable.green()));
            }
        }
        return msg;
    }

    fn tab_complete(&self, input: BashCompletionInput) -> Vec<String> {
        let current_option = self.get_current_option(input.args());

        if current_option.is_some() {
            let current_option = current_option.unwrap();
            if let OptionType::Options(suboptions) = &current_option.option_type {
                let autocomplete_options = create_strings_from_vector(suboptions);
                let completions = input.complete_subcommand(autocomplete_options);
                return completions;
            } else if let OptionType::Operation(_operation) = &current_option.option_type {
            }
        } else if current_option.is_none() {
            let autocomplete_options = create_strings_from_vector(&self.options);
            let completions = input.complete_subcommand(autocomplete_options);
            return completions;
        }
        return vec![];
    }

    fn get_current_option(&self, input: Vec<&str>) -> Option<CommandOption> {
        let mut current_option = None;
        let mut current_list = &self.options;
        for typed in &input {
            for option in current_list {
                if typed == &option.readable {
                    current_option = Some(option);
                    if let OptionType::Options(suboptions) = &current_option.unwrap().option_type {
                        // more to lookup, continue
                        current_list = suboptions;
                    } else if let OptionType::Operation(_operation) =
                        current_option.unwrap().option_type
                    {
                        //this is the last command
                        return current_option.cloned();
                    }
                }
            }
        }

        return current_option.cloned();
    }
}

fn create_strings_from_vector(options: &Vec<CommandOption>) -> Vec<&str> {
    let mut strings: Vec<&str> = Vec::new();
    for option in options {
        strings.push(option.readable.as_str());
    }
    return strings;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_build() {
        let _input = BashCompletionInput::from("er build ");
    }

    #[test]
    fn test_options() {
        let func = || 0;
        let options = vec![create_operation("foo", func)];
        let arguments = BashCompletionInput::from("er ");
        let autocompleter = Autocomleter { options: options };
        let current_option = autocompleter.get_current_option(arguments.args());

        // no option is returned
        assert!(current_option.is_none());

        let completions = autocompleter.tab_complete(arguments);

        assert!(completions.len() > 0);
        assert!(completions[0] == autocompleter.options[0].readable);
    }

    #[test]
    fn test_options_longer() {
        let func = || 0;
        let options = vec![create_option(
            "foobar",
            vec![create_operation("foo", func), create_operation("bar", func)],
        )];
        let arguments = BashCompletionInput::from("er foobar ");
        let autocompleter = Autocomleter { options: options };
        let current_option = autocompleter.get_current_option(arguments.args());

        // a option is returned
        assert!(current_option.is_some());

        let completions = autocompleter.tab_complete(arguments);

        assert!(completions.len() > 0);
        assert!(completions[0] == "foo");
        assert!(completions[1] == "bar");
    }
}
