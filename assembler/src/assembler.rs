use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    read_file("./file");
}



fn read_file(file_path : &str)  {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
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