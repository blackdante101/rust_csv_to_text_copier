use std::{
    env,
    fs::{self, File},
    io::{self, Write}

};

fn main() -> Result<(), io::Error> {
    println!("********************************************");
    println!("*** Extract data from a CSV to text file ***");
    println!("********************************************");

    if env::args().len() < 2{
       return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error please enter a valid argument eg. cargo run <file_path>",
        ))
    } else{

        let file_path = env::args().nth(1).unwrap();

        let content_of_csv = fs::read_to_string(&file_path)?;

        println!(".....processing file contents (0/1)");

        let output_file_name = "output.txt";

        let mut new_text_file = File::create(output_file_name)?;

        new_text_file.write(content_of_csv.as_bytes())?;

        println!(".....csv file has been copied successfully! (1/1)");
        println!("{} has been copied into {}", &file_path, output_file_name);

           
        }
        
        Ok(())
    }
 

