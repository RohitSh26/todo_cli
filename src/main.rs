mod input;
mod storage;
mod task;

use crate::input::get_user_input;
use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
use std::io;

enum Command {
    Add,
    List,
    Complete,
    Delete,
    Quit,
    Unknown(String),
}

impl Command {
    fn from(input: &str) -> Command {
        match input {
            "add" => Command::Add,
            "list" => Command::List,
            "complete" => Command::Complete,
            "delete" => Command::Delete,
            "quit" => Command::Quit,
            other => Command::Unknown(other.to_string()),
        }
    }
}

fn main() {
    println!("Welcome to Todo CLI!");

    // create a tasks vector
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!();

        let input = get_user_input("Please enter a command (add, list, complete, quit):");

        let command = Command::from(input.trim());

        // match keyword
        match command {
            Command::Add => {
                println!("Enter task description:");
                let mut description = String::new();

                io::stdin()
                    .read_line(&mut description)
                    .expect("Fail to read description");

                let description = description.trim().to_string();

                let task = Task {
                    description,
                    completed: false,
                };

                tasks.push(task);

                if let Err(e) = save_tasks(&tasks) {
                    println!("Error saving tasks: {}", e);
                }

                println!("Task added!!");
            }
            Command::List => {
                if tasks.is_empty() {
                    println!("No task found.")
                } else {
                    println!("Your tasks:");
                    for (index, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "✔" } else { "✘" };
                        println!("{}: [{}] {}", index + 1, status, task.description)
                    }
                }
            }
            Command::Complete => {
                if tasks.is_empty() {
                    println!("No task found to complete.")
                } else {
                    println!("Enter the task # (number) of the task you have completed:");

                    let mut input = String::new();

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Fail to read input");

                    let input = input.trim();

                    // convert input into a number
                    match input.parse::<usize>() {
                        Ok(num) => {
                            if num == 0 || num > tasks.len() {
                                println!("Invalid task number.");
                            } else {
                                tasks[num - 1].completed = true;

                                if let Err(e) = save_tasks(&tasks) {
                                    println!("Error saving tasks: {}", e);
                                }

                                println!("✔ Task {} marked as completed.", num);
                            }
                        }
                        Err(_) => println!("Please enter a valid number."),
                    }
                }
            }
            Command::Delete => {
                if tasks.is_empty() {
                    println!("No tasks to delete.");
                } else {
                    let input = get_user_input("Enter the number of the task you want to delete:");

                    match input.trim().parse::<usize>() {
                        Ok(num) => {
                            if num == 0 || num > tasks.len() {
                                println!("Invalid task number.");
                            } else {
                                let removed_task = tasks.remove(num - 1);
                                if let Err(e) = save_tasks(&tasks) {
                                    println!("Error saving tasks: {}", e);
                                }

                                println!("Task '{}' deleted.", removed_task.description);
                            }
                        }
                        Err(_) => println!("Please enter a valid number."),
                    }
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown(cmd) => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
