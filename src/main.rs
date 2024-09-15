use std::io;

fn main() {
    println!("Welcome to Todo CLI!");

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
            "add" => println!("You chose to add a new task."),
            "list" => println!("Listing all tasks."),
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command: {}", command),
        }
        
    }

}
