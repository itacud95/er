use std::{process::exit, vec};

use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    complete();
    println!("in main")
}

struct Command<'a> {
    command: &'a str,
    childs: Vec<&'a Command<'a>>,
}

impl<'a> Command<'a> {
    pub fn new(value: Box<&'a str>, cmds: Box<Vec<&'a Command>>) -> Self {
        Self {
            command: *value,
            childs: *cmds,
        }
    }
    pub fn get_sub_commands(&self) -> Vec<String> {
        let mut commands = vec![];
        for name in self.childs.iter() {
            //println!("{}::sub->{}", self.command, name.value());
            commands.push(name.value());
        }
        return commands;
    }
    pub fn contains(&self, cmd: &str) -> bool {
        for command in self.get_sub_commands() {
            if cmd == command {
                return true;
            }
        }
        return false;
    }
    pub fn get_string(&self, current_args: &Vec<&str>) -> Vec<String> {
        //println!("X{}", current_args.len());

        let mut num = 0;
        // for arg in current_args.iter().rev() {
        for arg in current_args {
            // println!("#{} {}#", num, arg);
            num += 1;

            for cmd in self.childs.iter() {
                if arg == &cmd.value() {
                    // println!("{}=={}", arg, &cmd.value());
                    // if (cmd.contains(arg))
                    // if (cmd.childs.len() > 1) {
                    return cmd.get_string(current_args);
                    // }
                    // return cmd.get_sub_commands();
                }
            }
        }

        // println!("EmptyReturn");
        return self.get_sub_commands();
        // return vec![];
    }
    pub fn value(&self) -> String {
        self.command.to_string()
    }
}

fn complete() {
    let input = BashCompletionInput::from_env();

    match input {
        Ok(file) => write_out(&file),
        Err(_) => return,
    };
}

// fn create_cmd_without_subcommands<'a>(cmd: &str) -> Command<'a> {
//     return Command::new(Box::new(cmd), Box::new(vec![]));
// }

fn write_out(input: &BashCompletionInput) {
    let sdk_basic_test_cmd = Command::new(Box::new("sdk_basic_test"), Box::new(vec![]));
    let elf_test_cmd = Command::new(Box::new("elf_test"), Box::new(vec![]));
    let obfuscated_ptr_test_cmd = Command::new(Box::new("obfuscated_test"), Box::new(vec![]));

    let shield_cmd = Command::new(
        Box::new("shield"),
        Box::new(vec![
            &elf_test_cmd,
            &sdk_basic_test_cmd,
            &obfuscated_ptr_test_cmd,
        ]),
    );

    let build_cmd = Command::new(Box::new("build"), Box::new(vec![&shield_cmd]));
    let log_cmd = Command::new(Box::new("log"), Box::new(vec![]));

    let current_args = input.args().clone();

    let root_cmd = Command::new(Box::new(""), Box::new(vec![&build_cmd, &log_cmd]));
    let output = root_cmd.get_string(&current_args);
    complete_string(input, output);

    // generate_from_args(args, commands);
    // let c = args.clone();

    // match input.arg_index() {
    //     _ => complete_string(input, generate_from_args(args, commands)),
    // }
    exit(0)
}

fn complete_string(input: &BashCompletionInput, txt: Vec<String>) {
    let mut commands = vec![""];
    for cmd in txt.iter() {
        //println!("comp::{}", cmd);
        commands.push(cmd.as_str())
    }

    // let c = input.args();
    // commands = c;

    let completions = input.complete_subcommand(commands);
    completions.suggest();
}
