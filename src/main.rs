mod reader;
mod string_utils;

use crate::reader::{ CSVReader };
use std::env::args;

fn main() {
    let args = Box::new(args().collect::<Vec<String>>()).leak();

    if args.len() == 1 {
        println!("Pass an argument!");
        return;
    }

    let filename = args[1].as_str();
    let filereader = CSVReader::new(filename);
    let file_content = filereader.read();
    
    match file_content {
        Ok(file_content) => {
            file_content.display().unwrap();
        },
        Err(error) => println!("{error}"),
    }
}
