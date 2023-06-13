use std::io::{self, Read, Write, BufReader, BufRead};
use std::fs::{File, write};
use std::fs::OpenOptions;

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
    //return input_string.trim().to_string();
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

    let _read_result = file.read_to_string(&mut content);
    
    return content;
  // let item_array:Vec<String> = content
  //     .split("\n")
  //     .map(|item| item.to_string())
  //     .collect();
  // return item_array;
}


fn main() {

    let mut list: Vec<String> = Vec::new();

    let saved_items = get_saved_list();
    list.push(saved_items);

    let mut todolist_file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("todolist.txt")
    {
        Ok(file) => file,
        Err(error) => panic!("Error creating file: {:?}", error),
    };
    

   // for saved_item in saved_items {
   //     list.push(saved_item);
   // }
    println!("{:?}", list);
    // here we get rid of all the empty strings
    list.retain(|s| !s.is_empty());
    
    println!("


████████╗ ██████╗     ██████╗  ██████╗ 
╚══██╔══╝██╔═══██╗    ██╔══██╗██╔═══██╗
   ██║   ██║   ██║    ██║  ██║██║   ██║
   ██║   ██║   ██║    ██║  ██║██║   ██║
   ██║   ╚██████╔╝    ██████╔╝╚██████╔╝
   ╚═╝    ╚═════╝     ╚═════╝  ╚═════╝ 
                                       

              ");

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
        
// the issue is we don't have newline characters written to the file
// so when reading the file, we don't see newlines. 
// without newlines we can't identify each item separately. 
            loop {
                let user_input = get_userinput_string();

                if user_input.contains("exit") {
                    break;
                } else {
                    list.push(user_input);
                }
            }
            // here we get rid of all the empty strings
            list.retain(|s| !s.is_empty());
            display_guide();
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

            // here we get rid of all the empty strings
            list.retain(|s| !s.is_empty());
            for item in &list{
                print!("{}", item);
            }
        }

        else if user_choice == 'd' {
            println!("Which item do you want to mark as completed?");
            let mut i = 1;
            for item in &list{
                println!("{}. {}", i, item);
                i += 1;
            }

            let mut index:usize;
            let mut user_input = get_userinput_int();
            index = user_input-1;

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

    for item in &list {
        let _writefile = write!(todolist_file, "{}", item);
    }
    let _file_flush = todolist_file.flush();
    println!("Bye!");

}
