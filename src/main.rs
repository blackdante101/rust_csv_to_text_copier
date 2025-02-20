use std::io::Write;
use std::{env, process};
use std::fs::{self, File};
fn main() {
    println!("********************************************");
    println!("*** Extract data from a CSV to text file ***");
    println!("********************************************");

    if env::args().len() < 2{
        println!("Error please enter a valid argument eg. cargo run <file_path> ");
        process::exit(0);
    }

    let file_path = env::args().nth(1).unwrap();

    let content_of_csv = fs::read_to_string(&file_path);

    match content_of_csv {
        Ok(_) => {
            println!(".....processing file contents (0/1)");
       
            let content_of_csv = content_of_csv.unwrap();
            let output_file_name = "output.txt";

            let mut new_text_file = File::create(output_file_name).expect("Error creating file");

            new_text_file.write(content_of_csv.as_bytes()).expect("Error writing to file");

            println!(".....csv file has been copied successfully! (1/1)");
            println!("{} has been copied into {}", &file_path, output_file_name);

        },
        Err(e) => {
            println!("{}", e);
        }
    }

}
