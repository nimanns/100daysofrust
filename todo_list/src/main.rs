mod task;

use task::Task;
use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("To-Do List:");
        for task in &tasks {
            println!("{}: {}", task.id, task.description);
        }

        println!("\nOptions:");
        println!("1. Add a task");
        println!("2. Delete a task");
        println!("3. Quit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");

                let task = Task::new(next_id, description.trim().to_string());
                tasks.push(task);
                next_id += 1;
            },
            "2" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read input");

                let id: usize = id_input.trim().parse().expect("Please enter a valid number");
                tasks.retain(|task| task.id != id);
            },
            "3" => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}
