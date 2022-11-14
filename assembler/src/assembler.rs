extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;

enum ErrorType {
    BadString,
    HarmlessErr,
}

struct label_data {
    label_string: String,
    memory_addr: u32,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    //Check that the program has the right amount of arguments
    if args.len() < 2 || args.len() > 2{
        panic!("Usage: ./assembler filename\n");
    }
    let file_path = &args[1]; 

    parse_file(file_path);
}


fn parse_file(file_path : &str)  {
     // Init table for labels
    let mut labels = Vec::new();

    // Index of line in file
    let index = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line{
                if line.len() > 0 {
                    

                    //Check for comments
                    let comment = if let Ok(i) = locate_comment(&line){
                        i
                    }
                    else{
                        //no comment found, set a standard value
                        let i= line.len() - 1;
                        i
                    };

                
                    
                    if let Ok(label) = locate_labels(string) {
                        let new_label = label_data {
                            label_string = label,
                            memory_addr = index,
                        }
                    } else
                    {
                        // No labels
                    }

                    //Take a slice of the line from start to where a comment was found
                    let line_slice = &line[..comment];
                    capture_command(line_slice);

                    index++;
                } 
            }
        }
    }else{
        panic!("File does not exist in current path!\n");
    }
}

/**
 * Captures commands from a text
 */
fn capture_command(text: &str){
    let re = Regex::new(r"(and) \s*\$([avtsk][0-9]|[0-9]|zero|at),\s*\$([avtsk][0-9]|[0-9]|zero|at),\s*\$([avtsk][0-9]|[0-9]|zero|at)").unwrap();
    for cap in re.captures_iter(text) {
        println!("Operation: {} Reg 1: {} Reg 2: {} Reg 3: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
    }    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn locate_comment(line: &str) -> Result<usize, ErrorType>{
    // Will itterate over "line" string and search for "#"
    // Will return first "#" found, if no "#" is found will return empty error.
    /*let index = Regex::new("#").unwrap().find(line).unwrap();
    match index {
        Option<Match<None>> => {
            return Err(error_type::harmless_err);
        }
        _=> {
        return Ok(index.start());
        }
    }*/
    for cap in Regex::new("#").unwrap().find_iter(line) {
        return Ok(cap.start());
    }
    return Err(ErrorType::HarmlessErr);
}

fn locate_labels(line: &str) -> Result<String, ErrorType> {
    for cap in Regex::new("([a-z]|[A-z]|[0-9])+[:]").unwrap().find_iter(line) {
        if cap.start() == 0 {
            println!("{:?}", cap);
            return Ok(line[0..cap.end()].to_string());
            
        }
    }
    return Err(ErrorType::BadString);
}