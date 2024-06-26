# Todo List in Rust
This Rust project defines a simple, command-line todo list application. It showcases the use of structs, traits, and a HashMap to manage todo items. Below is a comprehensive explanation of the code and its functionality.

## Overview
The application is structured around a TodoList struct and a Todo trait. The TodoList struct is used to represent the todo list, storing todo items in a HashMap where each item is associated with a unique number. The Todo trait defines operations that can be performed on the todo list, such as adding, removing, listing, completing, and uncompleting todo items.

### Struct: TodoList
Purpose: Represents a todo list.
Key Component: Uses a HashMap<u32, String> to store todo items, where the key (u32) is a unique number associated with each todo item, and the value (String) is the todo item's description.

### Trait: Todo
Purpose: Defines a set of operations for managing todo items.

### Operations:
add(&mut self, todo: String): Adds a new todo item to the list.
remove(&mut self, todo_number: u32): Removes a todo item from the list based on its unique number.
list(&self): Lists all todo items.
complete(&mut self, todo_number: u32): Marks a todo item as completed.
uncomplete(&mut self, todo_number: u32): Marks a todo item as uncompleted.
print(&self): Prints the todo list.

## Implementation of Todo for TodoList
Add Operation: Generates a unique number for the new todo item by taking the length of the current list and adding 1. Inserts the new todo item into the HashMap.
Remove Operation: Removes a todo item from the HashMap based on its unique number.

### How It Works
1. Adding a Todo Item: When a user adds a todo item, the application generates a unique number for the item by incrementing the current size of the HashMap. This number is used as the key to insert the todo item into the HashMap.

2. Removing a Todo Item: To remove an item, the application simply removes the entry from the HashMap using the unique number provided by the user.

## Conclusion
This Rust application provides a basic framework for a command-line todo list, demonstrating the use of structs, traits, and HashMap for managing a collection of items. The implementation of the Todo trait for the TodoList struct allows for easy management of todo items, including adding, removing, and listing operations.


# Running the Project
-------------------
To run this todo list application, you need to have Rust and Cargo installed on your system. If you haven't installed Rust and Cargo yet, you can download them from [the official Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, follow these steps:

1. Open a terminal or command prompt.
2. Navigate to the directory where you have saved this project.
3. Compile the project by running `cargo build`. This command compiles the project and generates an executable in the `target/debug` directory.
4. To run the application, execute `cargo run` from the project's root directory. This command compiles (if necessary) and runs the application.

### Example Commands
- To add a todo item: `cargo run add "Your todo item here"`
- To list all todo items: `cargo run list`
- To mark a todo item as completed: `cargo run complete <todo_number>`
- To remove a todo item: `cargo run remove <todo_number>`

Replace `<todo_number>` with the actual number of the todo item you wish to mark as completed or remove.