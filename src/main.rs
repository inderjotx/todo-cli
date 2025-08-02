use std::io;
pub mod todo_manager;
pub mod utils;

use todo_manager::{Status, Todo, TodoManager};
use utils::*;

fn main() {
    let mut manager = TodoManager { todos: Vec::new() };

    println!("----------- Welcome to the TODO manager -------------- ");
    display_all_command();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error reading command");

        match command.trim() {
            "-h" => display_all_command(),
            "-a" => add_todo(&mut manager),
            "-d" => mark_done(&mut manager),
            "-r" => remove_todo(&mut manager),
            "-all" => list_todo(&manager),
            "-dd" => list_done(&manager),
            "-nd" => list_not_done(&manager),
            _ => println!("This is weird"),
        }
    }
}
