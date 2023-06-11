use std::io;

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


fn main() {

    let mut list = TodoList::new();

    loop {

        println!("
                 Press 'a' to add a new item to the list.
                 Press 'e' to view the items.
                 Press 'd' to mark a completed item.
                 Press 'D' to delete an item.
                 Press 'q' to quit."
                );

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

        if user_choice == 'e' {
            println!("Your todo list");
            println!("--------------");

            let items = list.get_items();
            for item in items {
                print!("{}", item);
            }
        }

    }
    println!("Bye!");

}
