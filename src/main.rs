use std::io::{self, Read};
use std::fs::File;

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

fn get_userinput_int () -> usize {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Can't read values!");
    let number:usize = match input_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return 0;
        }
    };
    return number;
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

fn get_saved_list () -> String {
    let file_path = "todolist.txt";
    let mut content = String::new();
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Can't read file! {:?}", error),
    }; 
    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(error) => panic!("Something went wrong while reading! {:?}", error)
    };

    return content;
}


fn main() {

    let mut list: Vec<String> = Vec::new();

    display_guide();

    let mut todolist_file = match File::create("todolist.txt") {
        Ok(file) => file,
        Err(error) => panic!("Error creating file: {:?}", error),
    };
    
    let saved_item = get_saved_list();
    println!("Saved stuff: {}", saved_item);
    
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
                    list.push(user_input);
                }

            }
        }

        else if user_choice == 'e' {
            println!("Your todo list");
            println!("--------------");

            if list.len() == 0 {
                println!("
                Your list is empty!
                Press 'a' 
                to add a new item to the list."
                )
            }

            for item in &list{
                print!("{}", item);
            }
        }

        else if user_choice == 'd' {
            println!("Which item do you want to mark as completed?");
            let mut i = 1;
            for item in &list{
                print!("{}. {}", i, item);
                i += 1;
            }
            // Try to save completed tasks into a separate file
            // And remove it from the todo list.
            // [1, 2, 3, 4]
            let mut index:usize = 0;
            let mut user_input = get_userinput_int();
            while user_input <= 0 || user_input > list.len() {
                println!("Please enter a valid value: ");
                user_input = get_userinput_int();
                index = user_input as usize - 1;
            }
            list.remove(index);
        }

        else if user_choice == 'h' {
            display_guide();
        }

    }
    println!("Bye!");

}
