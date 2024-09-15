use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    // The ? operator after each file operation propagates errors, simplifying error handling.
    //The ? operator propagates any errors.

    // Converts the tasks vector into a JSON String.
    let serialized = serde_json::to_string(tasks)?;

    // Creates a new file named tasks.json, overwriting if it exists.
    // Returns a File object.
    let mut file = File::create("tasks.json")?;

    // Writes the serialized JSON data to the file.
    // as_bytes() converts the String into a byte array.
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn load_tasks() -> Vec<Task> {
    let data = match fs::read_to_string("tasks.json") {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // File doesn't exist, return an empty vector
                return Vec::new();
            }
            other_error => {
                panic!("Problem reading the file: {:?}", other_error);
            }
        },
    };

    serde_json::from_str(&data).unwrap_or_else(|_| {
        println!("Error parsing tasks file. Starting with an empty task list.");
        Vec::new()
    })
}

fn main() {
    println!("Welcome to Todo CLI!");

    // create a tasks vector
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("Please enter a command (add, list, quit):");

        let mut command = String::new();

        // read std input stream
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        // trim read command
        let command = command.trim();

        // match keyword
        match command {
            "add" => {
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
            "list" => {
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
            "complete" => {
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
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command: {}", command),
        }
    }
}
