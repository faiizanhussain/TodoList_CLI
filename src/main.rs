use std::collections::HashMap;
use std::io::{self, Write};

// Define a struct to represent a todo list
struct TodoList {
    list: HashMap<u32, String>, // HashMap to store todo items with their corresponding numbers
}

// Define a trait for todo operations
trait Todo {
    fn add(&mut self, todo: String); // Add a todo item to the list
    fn remove(&mut self, todo_number: u32); // Remove a todo item from the list
    fn list(&self); // List all todo items
    fn complete(&mut self, todo_number: u32); // Mark a todo item as completed
    fn uncomplete(&mut self, todo_number: u32); // Mark a todo item as uncompleted
    fn print(&self); // Print the todo list
}

// Implement the Todo trait for the TodoList struct
impl Todo for TodoList {
    fn add(&mut self, todo: String) {
        let todo_number = self.list.len() as u32 + 1; // Generate a unique todo number
        self.list.insert(todo_number, todo); // Insert the todo item into the list
    }

    fn remove(&mut self, todo_number: u32) {
        self.list.remove(&todo_number); // Remove the todo item from the list
    }

    fn list(&self) {
        self.print(); // Print the todo list
    }

    fn complete(&mut self, todo_number: u32) {
        if let Some(todo) = self.list.get_mut(&todo_number) {
            *todo = format!("{} - Completed", todo); // Mark the todo item as completed
        }
    }

    fn uncomplete(&mut self, todo_number: u32) {
        if let Some(todo) = self.list.get_mut(&todo_number) {
            *todo = todo.replace(" - Completed", ""); // Mark the todo item as uncompleted
        }
    }

    fn print(&self) {
        if self.list.is_empty() {
            println!("\nTodo list is empty.\n"); // Display a message indicating that the list is empty
        } else {
            println!("Todo List\n"); // Print a new line (for better formatting

            for (todo_number, todo) in &self.list {
                println!("{}: {}", todo_number, todo); // Print each todo item with its number
            }

            println!("\n"); // Print a new line (for better formatting
        }
    }
}

fn main() {
    let mut todo_list = TodoList {
        list: HashMap::new(), // Create a new empty todo list
    };

    loop {
        println!("Menu:");
        println!("1. Add a todo item");
        println!("2. Remove a todo item");
        println!("3. List all todo items");
        println!("4. Mark a todo item as completed");
        println!("5. Mark a todo item as uncompleted");
        println!("6. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = match choice.trim().parse::<u32>() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid input. Please enter a valid choice.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter the todo item: ");
                io::stdout().flush().unwrap();
                let mut todo = String::new();
                io::stdin().read_line(&mut todo).unwrap();
                todo_list.add(todo.trim().to_string()); // Add a new todo item
            }
            2 => {
                print!("Enter the todo item number to remove: ");
                io::stdout().flush().unwrap();
                let mut todo_number = String::new();
                io::stdin().read_line(&mut todo_number).unwrap();
                let todo_number = match todo_number.trim().parse::<u32>() {
                    Ok(todo_number) => todo_number,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid todo item number.");
                        continue;
                    }
                };
                todo_list.remove(todo_number); // Remove a todo item
            }
            3 => {
                todo_list.list(); // List all todo items
            }
            4 => {
                print!("Enter the todo item number to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut todo_number = String::new();
                io::stdin().read_line(&mut todo_number).unwrap();
                let todo_number = match todo_number.trim().parse::<u32>() {
                    Ok(todo_number) => todo_number,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid todo item number.");
                        continue;
                    }
                };
                todo_list.complete(todo_number); // Mark a todo item as completed
            }
            5 => {
                print!("Enter the todo item number to mark as uncompleted: ");
                io::stdout().flush().unwrap();
                let mut todo_number = String::new();
                io::stdin().read_line(&mut todo_number).unwrap();
                let todo_number = match todo_number.trim().parse::<u32>() {
                    Ok(todo_number) => todo_number,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid todo item number.");
                        continue;
                    }
                };
                todo_list.uncomplete(todo_number); // Mark a todo item as uncompleted
            }
            6 => {
                println!("Exiting...");
                break; // Exit the program
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
