use std::io;

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    println!("Welcome to Todo CLI!");

    // create a tasks vector
    let mut tasks: Vec<Task> = Vec::new();

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
