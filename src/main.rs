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
        return &self.todos
    }
}


fn main() {
    println!("
              Press 'a' to add a new item to the list.
              Press 'e' to view the items.
              Press 'd' to mark a completed item.
              Press 'D' to delete an item.
              Press 'q' to quit.");

    let mut list = TodoList::new();
    
    loop {

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Can't read values!");

        let user_choice = match user_input.chars().next() {
            Some(c) => c,
            None => {
                println!("no character entered");
                return;
            }
        };

        if user_choice == 'q' {
           break; 
        }
        else if user_choice == 'a' {
            print!("Enter the item you want to add.\n>");
            let mut new_item = String::new();
            io::stdin()
                .read_line(&mut new_item)
                .expect("Can't read values!");
            list.add_item(new_item); 
        }
        else if user_choice == 'e' {
            let items = list.get_items();
        }
        
    }

}
