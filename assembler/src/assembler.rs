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


enum InstructionType{
    R,
    I1,
    I2,
    J1,
    J2
}
/* 
struct command{
    capture : Captures,
    regex : String,

}*/


struct table_instruction{
    command : str,
    machine_code : u32
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

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line{
                if line.len() > 0 {
                    let regex :String;
                    let type: InstructionType;

                    //Check for comments
                    let comment = if let Ok(i) = locate_comment(&line){
                        i
                    }
                    else{
                        //no comment found, set a standard value
                        let i= line.len() - 1;
                        i
                    };

                
                    /* 
                    if let Ok(label) = locate_labels(string) {
                        // Save label
                    } else
                    {
                        // No labels
                    }*/

                    //Take a slice of the line from start to where a comment was found
                    let line_slice = &line[..comment];
                    (regex, type) = identify_type(line_slice).unwrap();
                    
                    Captures cap = capture_command(line_slice, &regex);


                    match type
                    
                } 
            }
        }
    }else{
        panic!();
    }
}



fn identify_type(text: &str)->Option<(String, InstructionType)>{
    let r_type = Regex::new(r"(and|sub|nor|or|and|slt)").unwrap();
    let i1_type = Regex::new(r"(addi|beq)").unwrap();
    let i2_type = Regex::new(r"(lw|sw)").unwrap();
    let j_type = Regex::new(r"(j|jr)").unwrap();

    if r_type.is_match(text) {
        return ("(and|sub|nor|or|and|slt)) \s*\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]|[0-9]+|zero|at)".to_string(), R);
    }
    else if i1_type.is_match(text){
        return ("(addi|beq) \s*\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]+|[0-9]|zero|at),\s*-*[0-9]+".to_string(), I1);
    }
    else if i2_type.is_match(text){
        return ("(lw|sw) \s*\$([avtsk][0-9]|[0-9]+|zero|at),\s*([0-9]*)\(\$([avtsk][0-9])\)".to_string(), I2);
    }
    else if j1_type.is_match(text){
        return ("(j)\s+\w+",J1);
    }else if j2_type.is_match(text){
        return ("(jr) ",J2);
    }else{
        return None;
    }
    
}

/**
 * Captures commands from a text
 */
fn capture_command(text: &str, regex:&str) -> Captures{
    let re = Regex::new(regex).unwrap();
    re.captures(text).

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
