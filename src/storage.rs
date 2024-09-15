use crate::task::Task;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};

pub fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let serialized = serde_json::to_string(tasks)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> Vec<Task> {
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
