use colored::*;
use dialoguer::{Input, Select};

pub fn print_title() {
    println!("{}", "[proc_editor-rs]".bold().green());
}

pub fn error(msg: &str) {
    println!("{} {}", "[ERROR]".red(), msg.red());
}

pub fn info(msg: &str) {
    println!("{} {}", "[INFO]".blue(), msg.blue());
}

pub fn prompt_package_name() -> String {
    Input::new()
        .with_prompt("Enter package name")
        .interact_text()
        .unwrap()
}

pub fn main_menu() -> usize {
    let items = vec![
        "Environment Variables",
        "Open File Descriptors",
        "Memory Maps",
        "Direct Memory Modification",
        "Exit"
    ];
    Select::new()
        .items(&items)
        .default(0)
        .interact()
        .unwrap() + 1
}
