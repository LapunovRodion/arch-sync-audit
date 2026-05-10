use std::env;
use std::process::Command;

pub fn command_output_lines(command: &str, args: &[&str]) {
    let _output = Command::new(command).args(args).output();
}

pub fn current_shell() -> Option<String> {
    env::var("SHELL").ok()
}
