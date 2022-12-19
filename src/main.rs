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
            commands.push(name.value());
        }
        return commands;
    }
    pub fn get_commands(&self) -> Vec<&str> {
        let mut commands = vec![self.command];
        // let mut commands = vec![self.command];
        // let mut commands = String::from(self.command);

        for name in self.childs.iter() {
            commands.append(&mut name.get_commands())
            // commands.push_str(", ");
            // commands.push_str(name.get_commands().as_str());
        }
        return commands;
    }
    pub fn add_sub_command(&mut self, cmd: &'a Command<'a>) {
        self.childs.push(cmd);
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

fn create_cmd_without_subcommands<'a>(cmd: &str) -> Command<'a> {
    return Command::new(Box::new("sdb_basic_test"), Box::new(vec![]));
}

fn generate_from_index(index: u64, commands: Vec<Command>) -> Vec<String> {
    let mut output = Vec::new();
    for cmd in commands.iter() {
        output.push(cmd.value())
    }

    return output;
}

fn generate_from_args(args: Vec<&str>, commands: Vec<Command>) -> Vec<String> {
    let currentArg = args.first().expect("It is not here?");

    // println!("X{}", args.len());
    // for arg in &args {
    //     println!("#{}#", arg);
    // }

    let mut output = Vec::new();

    if args.len() < 4 {}

    for cmd in commands.iter() {
        if currentArg == &cmd.value() {
            return cmd.get_sub_commands();
        }
        // return cmd.get_sub_commands();
        // output.push(cmd.value())
        // let search = cmd.childs;
        // search.contains();
    }

    for cmd in commands.iter() {
        output.push(cmd.value())
        // return cmd.get_sub_commands()
    }

    return output;
}

fn write_out(input: &BashCompletionInput) {
    let sdk_basic_test_cmd = create_cmd_without_subcommands("sdk_basic_test");
    let elf_test_cmd = create_cmd_without_subcommands("elf_test");
    let obfuscated_ptr_test_cmd = create_cmd_without_subcommands("obfuscated_ptr_test");

    let shield_cmd = Command::new(
        Box::new("shield"),
        Box::new(vec![
            &sdk_basic_test_cmd,
            &elf_test_cmd,
            &obfuscated_ptr_test_cmd,
        ]),
    );

    let build_cmd = Command::new(Box::new("build"), Box::new(vec![&shield_cmd]));
    let log_cmd = Command::new(Box::new("log"), Box::new(vec![]));

    let commands = vec![build_cmd, log_cmd];

    let args = input.args().clone();
    // generate_from_args(args, commands);
    // let c = args.clone();

    match input.arg_index() {
        _ => complete_string(input, generate_from_args(args, commands)),
        // _ => complete_string(input, generate_from_index(0, commands)),
        // _ => complete_string(input, build_cmd.get_sub_commands()),
        // 1 => complete_string(input, Vec::from_iter(commands[0..1].iter().cloned())),
        // _ => complete_string(input, Vec::from_iter(commands[1..2].iter().cloned())),
    }
    exit(0)
}

fn complete_string(input: &BashCompletionInput, txt: Vec<String>) {
    let mut commands = vec![""];
    for cmd in txt.iter() {
        commands.push(cmd.as_str())
    }

    // let c = input.args();
    // commands = c;

    let completions = input.complete_subcommand(commands);
    completions.suggest();
}
