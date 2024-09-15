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

        // case when user quits
        if command == "quit" {
            println!("Goodbye!!");
            break;
        }
        else {
            println!("You entered {} Command", command);
        }
    }

}
