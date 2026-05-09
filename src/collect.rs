use std::env;

pub fn current_shell() -> Option<String> {
    env::var("SHELL").ok()
}
