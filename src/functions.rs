use std::io;
use std::io::Write;
use std::fs::File;
use std::panic;


/* Functions for file i/o program */

pub fn print_file(path: &str){
    /* Reads a file and prints it to the console */
    let output = std::fs::read_to_string(path).unwrap_or(String::from("ERROR FILE NOT FOUND"));

    for line in output.lines(){
        println!("{line}");
    }

}

pub fn write_file(name: &str, user_input: &str ){
    /* Create a new file or overwite an exsisting one */
    let my_file: Result<File, io::Error> = File::create(name);

    let mut new_file = match my_file{
        Ok(file) => file,
        _ =>{
            panic!("Failed to write file")
        }
    };

    write!(new_file, "{user_input}")
        .expect("Failed to write file");

}

pub fn get_user_input() -> String{

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("No input");
    input
}