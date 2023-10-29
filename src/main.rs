mod functions;
use functions::print_file;
use functions::write_file;
use functions::get_user_input;

fn main(){
    let mut running = true;
    let mut choice = String::new();
    let mut option = "";
    let mut path = "";
    let mut input = "";
    let option1 = "1";
    let option2 = "2";
    let option3 = "3";

    let options = "Choose an option (1-3):\n  1. Read a file and print it to console\n  2. Create or truncate a file\n  3. Quit Program";
    println!("Welcome to Zach's simple file reader and writer!");

    while running {
        
        println!("\n{options}");
        choice = get_user_input();
        option = choice.as_str().trim_end();


            if option1.eq(option) == true{
                println!("Enter File Path: ");
                let file_path = get_user_input();
                path = file_path.as_str().trim_end();
                print_file(path);
            }
                
            else if option2.eq(option) == true{
                println!("Enter File Path: ");
                let file_path = get_user_input();
                path = file_path.as_str().trim_end();
                println!("Enter what you want to write: ");
                let file_input = get_user_input();
                input = file_input.as_str().trim_end();
                write_file(path, &input);
            }
            else if option3.eq(option) == true{
                print!("Goodbye!");
                running = false;
            }
            else{
                println!("ERROR INVALID INPUT, ENTER AGAIN");
            }
        };
    }
