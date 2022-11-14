extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;
/* 
enum error_type {
    bad_string,
    harmless_err,
}

struct instruction{
    command : str,
    machine_code : u32
}*/


fn main() {
   // let string = "Hee:asdds";
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    read_file(file_path);

/* 
    if let Ok(index) = locate_comment(string) {
        println!("{}", index);
    }
    
    if let Ok(label) = locate_labels(string) {
        // Save label
    } else
    {
        // No labels
    }
*/
    
}


fn read_file(file_path : &str)  {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let re = Regex::new(r"(add) (\s*)\$(([avtsk][0-9])|[0-9]|zero|at),(\s*)\$(([avtsk][0-9])|[0-9]|zero|at),(\s*)\$(([avtsk][0-9])|[0-9]|zero|at)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!("Operation: {} Reg 1: {} Reg 2: {} Reg 3: {}", &cap[1], &cap[3], &cap[6], &cap[9]);
                }                   
               
            }
        }
    }else{
        panic!("File does not exist in current path!\n");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
/* 
fn locate_comment(line: &str) -> Result<usize, error_type>{
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
    return Err(error_type::harmless_err);
}

fn locate_labels(line: &str) -> Result<String, error_type> {
    for cap in Regex::new("([a-z]|[A-z]|[0-9])+[:]").unwrap().find_iter(line) {
        if cap.start() == 0 {
            println!("{:?}", cap);
            return Ok(line[0..cap.end()].to_string());
            
        }
    }
    return Err(error_type::bad_string);
}*/