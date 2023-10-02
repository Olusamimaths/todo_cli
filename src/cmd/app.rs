use colored::Colorize;
use std::io;

pub struct App {}

impl App {
    pub fn new() -> Self {
        return App {};
    }

    pub fn init(&self) {
        println!("{}", "-----------------------------------------".cyan());
        println!("{} ", "\t⌛ Welcome to the Todo CLI".cyan());
        println!("{}", "-----------------------------------------".cyan());

        println!("{}", "What do you want to do today?".italic().yellow());
        println!("{}", "    1. Create new task".cyan());
        println!("{}", "    2. View all tasks".cyan());
        println!("{}", "    3. Delete a task".cyan());
        println!("{}", "    4. Mark a task as complete".cyan());
        
        println!("{}", "Enter your choice: ".blink());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}
