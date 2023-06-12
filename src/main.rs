use std::io;
use std::convert::TryInto;

struct TodoList {
    todos: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
        }
    }

    fn add_item(&mut self, todo: String) {
        self.todos.push(todo);
    }

    fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.todos.len() {
            Some(self.todos.remove(index))
        } else {
            None
        }
    }

    fn get_items(&self) -> &[String] {
        return &self.todos;
    }
}

fn get_userinput_character () -> char {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Can't read values!");

    let user_input_char = match input_string.chars().next() {
        Some(c) => c,
        None => {
            println!("No character entered!");
            return 'a'; 
        }
    };
    return user_input_char;
}

fn get_userinput_string () -> String {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Can't read values!");
    return input_string;
}

fn get_userinput_int () -> i32 {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Can't read values!");
    let number: i32 = match input_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return -1;
        }
    };
    return number;
}

fn get_items (list:&TodoList) -> &[String] {
    let items = list.get_items();
    return &items;
}

fn display_guide() {
    println!("
             Press 'a' to add a new item to the list.
             Press 'e' to view the items.
             Press 'd' to to remove the item from the list.
             Press 'q' to quit.
             Press 'h' to view this."
            );

}


fn main() {

    let mut list = TodoList::new();

    display_guide();
    
    loop {
        
        println!("--> ");

        let user_choice: char;
        user_choice = get_userinput_character();
        
        if user_choice == 'q' {
            break;
        }

        if user_choice == 'a' {
            println!("Add items to the list");
            println!("---------------------");
            println!("(Type 'exit' to go back)");
        
            loop {
                let user_input = get_userinput_string();

                if user_input.contains("exit") {
                    break;
                } else {
                    list.add_item(user_input);
                }

            }
        }

        else if user_choice == 'e' {
            println!("Your todo list");
            println!("--------------");

            let items = list.get_items();
            
            if items.len() == 0 {
                println!("
                Your list is empty!
                Press 'a' 
                to add a new item to the list."
                )
            }

            for item in items {
                print!("{}", item);
            }
        }

        else if user_choice == 'd' {
            println!("Which item do you want to mark as completed?");
            let mut i = 1;
            let items = get_items(&list);
            for item in items {
                print!("{}. {}", i, item);
                i += 1;
            }
            // Try to save completed tasks into a separate file
            // And remove it from the todo list.
            // [1, 2, 3, 4]
            let mut index:usize = 0;
            let mut user_input:i32 = get_userinput_int();
            while user_input <= 0 || user_input > items.len().try_into().unwrap() {
                println!("Please enter a valid value: ");
                user_input = get_userinput_int();
                index = user_input as usize - 1;
            }
            list.remove_item(index);
        }

        else if user_choice == 'h' {
            display_guide();
        }

    }
    println!("Bye!");

}
