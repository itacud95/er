use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};
use std::vec;

pub fn autocomplete() -> Option<fn() -> i32>{
    match BashCompletionInput::from_env() {
        Err(_) => {
            let args: Vec<String> = std::env::args().collect();
            let options = initialize_options();
            let v8: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
            let current_option = get_current_option(v8, &options);
            if current_option.is_none(){
                print_help();
                return None;
            }
            if let OptionType::Operation(operation) = &current_option.unwrap().option_type {
                return Some(operation.to_owned());
            }
            return None
        },
        Ok(input) => return run_autocomplete(&input),
    };
}

fn print_help() {
    let options = initialize_options();
    let options = create_strings_from_vector(&options);
    println!("Usage: er [OPTIONS]");
    println!("Options: ");
    for opt in options {
        println!("\t{0}", opt);
    }
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
            // return Some(*operation);
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

fn initialize_options() -> Vec<CommandOption> {
    vec![
        create_option(
            "binaries",
            vec![
                create_option(
                    "show",
                    vec![create_operation("test-file", test_function)],
                ),
                create_option("write", vec![create_operation("new-file.bin", test_function)]),
            ],
        ),
        create_operation("gnu-plot", test_function),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_build() {
        let _input = BashCompletionInput::from("er build "); // todo: Why do we need space after build?
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
                let _autocomplete_options = create_strings_from_vector(suboptions);
            }
            else if let OptionType::Operation(operation) = &current_option.option_type {
                let _func = operation;
            }
        }
        
        // let autocomplete_options = create_strings_from_vector(&options);
        // assert!(autocomplete_options.len() > 2);
    }
}
