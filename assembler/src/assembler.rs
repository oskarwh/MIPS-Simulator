extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;
use std::collections::hash_map;

const RS_POS: u32 = 21;
const RT_POS: u32 = 16;
const RD_POS: u32 = 11;
const SHAMT_POS: u32 = 6;

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

struct UndefinedLabel{
    row_index: u32,
    label_string: String,
}


fn main() {

    let args: Vec<String> = env::args().collect();

    
    //Check that the program has the right amount of arguments
    if args.len() < 2 || args.len() > 2{
        panic!("Usage: ./assembler filename\n");
    }
    let file_path = &args[1]; 

    let parsed_file = parse_file(file_path);
}

fn parse_file(file_path : &str) -> (Vec<u32>, Vec<String, bool>) {

    let mut registers = hash_map::HashMap::new();
    let mut instruction = hash_map::HashMap::new();
    setup_registers_table(&mut registers);
    setup_instruction_table(&mut instruction);
    // Init table for labels
    let mut labels = hash_map::HashMap::new();

    // Init vector for rows with undefined labels
    let mut undefined_j = Vec::new();
    // Init vector for storing generated machine code & assembler code
    let mut machine_code = Vec::new();
    let mut assembler_code = Vec::new();

    //println!("{} {}", registers.get(&"zero").unwrap(), instruction.get(&"add").unwrap());
    
    // Index of line in file
    let mut index = 0;
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

                    // Locate label on line if it exists.
                    if let Ok(label) = locate_labels(&line) {
                        labels.insert(label, index);
                    }

                    //Take a slice of the line from start to where a comment was found
                    let line_slice = &line[..comment];
                    // Bool to check if line contains machine code
                    let mut contain_code = true;
                    
                    // If the line contains an identifyable command, assemble the line to machine code and push it to vector
                    if let Some(regex, inst_type) = identify_type(line_slice){
                        let cap = capture_command(line_slice, &regex);
                        let line_code = match inst_type {
                            R => assemble_r_type(cap),
                            I1 => assemble_i_type(cap),
                            I2 => assemble_i_type(cap),
                            J1 => assemble_j1_type(index, cap, &mut labels, &mut undefined_j, &instruction),
                            J2 => assemble_j2_type(cap),registers : hash_map::HashMap<&'static str ,u32>,
                        };
                        machine_code.push(line_code); 
                    }else{
                        contain_code = false;    
                    }
                    // Push tuple to assembler code vector.
                    assembler_code.push((line, contain_code));
                    index += 1; 
                }
        }
    }else{
        panic!();
    }

    (machine_code, assembler_code)
}


fn assemble_r_type(cap:Captures, registers : hash_map::HashMap<&'static str ,u32>, instructions : hash_map::HashMap<&'static str ,u32>) -> u32{
    let cmnd = cap[1];
    let rs = cap[2];
    let reg2 = cap[3];
    
    let mut instr = instructions.get(&cmnd).unwrap();

    // Add RS
    instr = instr | if let Some(i) = registers.get(&rs){ 
        i<<RS_POS
    } else {
        // Register not found in table, use data as register number
        let i = rs.parse::<u32>();
        if i == Err || i.unwrap() > 31{
            //error
        }else{
            i.unwrap()<<RS_POS
        }
    };Stringster number
        let i = rs.parse::<u32>().unwrap();
        if i > 31{
            //error
        }else{
            i<<RT_POS
        }
    };


}

fn assemble_i_type() -> u32{

}


fn assemble_j1_type(index:u32, cap:Captures, labels: &mut hash_map::HashMap<String, u32>, undefined_j: &mut vector, instructions : &hash_map::HashMap<&'static str ,u32>) -> u32{
    let cmnd = cap[1];
    let dest = cap[2];
    let mut addr: u32 = 0;

    let mut instr = instructions.get(&cmnd).unwrap();
    
    if let Some(i) = labels.get(dest) {
        addr = i;
    }else {
        let temp = UndefinedLabel {
            row_index: index,
            label_string: dest
        }
        undefined_j.push(temp);
    }


    instr = instr | addr
} 

fn assemble_j2_type(cap:Captures, registers : hash_map::HashMap<&'static str ,u32>, instructions : hash_map::HashMap<&'static str ,u32>) -> u32{
    let cmnd = cap[1];
    let dest = cap[2];

    let mut instr = instructions.get(&cmnd).unwrap();

    instr = instr | if let Some(i) = registers.get(&dest).unwrap() {
        // found Register
        i
    }else {
        // register not found in table use data as register number
        let i = i.parse::<u32>();
        if i ==Err || i.unwrap() > 31{
            //error
        }else{
            i.unwrap()
        }
    }

    instr
} 




fn identify_type(text: &str)->Option<(String, InstructionType)>{
    let r_type = Regex::new(r"(and|sub|nor|or|and|slt)").unwrap();
    let i1_type = Regex::new(r"(addi|beq)").unwrap();
    let i2_type = Regex::new(r"(lw|sw)").unwrap();
    let j1_type = Regex::new(r"(j)").unwrap();
    let j2_type = Regex::new(r"(jr)").unwrap();

    if r_type.is_match(text) {
        return Some((r"(and|sub|nor|or|and|slt)\s+\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra),\s*\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra),\s*\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra)".to_string(), InstructionType::R));
    }
    else if i1_type.is_match(text){
        return Some((r"(addi)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]+|[0-9]|zero|at),\s*-*[0-9]+".to_string(), InstructionType::I1));
    }
    else if i2_type.is_match(text){
        return Some((r"(beq)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]+|[0-9]|zero|at),\s*\w+".to_string(), InstructionType::I1));
    }
    else if i3_type.is_match(text){
        return Some((r"(lw|sw)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*([0-9]*)\(\$([avtsk][0-9])\)".to_string(), InstructionType::I2));
    }
    else if j1_type.is_match(text){
        return Some((r"(j)\s+\w+".to_string(),InstructionType::J1));
    }else if j2_type.is_match(text){
        return Some((r"(jr)\s+\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra)".to_string(),InstructionType::J2));
    }else{
        return None;
    }
    
}

/**
 * Captures commands from a text
 */
fn capture_command<'a>(text: &'a str, regex:&'a str) -> regex::Captures<'a>{
    let re = Regex::new(regex).unwrap();
    re.captures(text).unwrap()

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

fn write_files(machine_code: mut Vec<String>, assembler_code: mut Vec<String>) {
    let mut listing_file = File::open("asm_listing")?;
    let mut machine_file = File::open("asm_instr")?;
    
    for machine_line in machine_file.iter() {
        
    }
}

fn setup_registers_table(registers: &mut hash_map::HashMap<&'static str ,u32>){
   
    registers.insert("zero", 0);
    registers.insert("at", 1);
    registers.insert("v0", 2);
    registers.insert("v1", 3);

    registers.insert("a0", 4);
    registers.insert("a1", 5);
    registers.insert("a2", 6);
    registers.insert("a3", 7);

    registers.insert("t0", 8);
    registers.insert("t1", 9);
    registers.insert("t2", 10);
    registers.insert("t3", 11);
    registers.insert("t4", 12);
    registers.insert("t5", 13);
    registers.insert("t6", 14);
    registers.insert("t7", 15);
    
    registers.insert("s0", 16);
    registers.insert("s1", 17);
    registers.insert("s2", 18);
    registers.insert("s3", 19);
    registers.insert("s4", 20);
    registers.insert("s5", 21);
    registers.insert("s6", 22);
    registers.insert("s7", 23);

    registers.insert("t8", 24);
    registers.insert("t9", 25);
    registers.insert("k0", 26);
    registers.insert("k1", 27);
    registers.insert("gp", 28);
    registers.insert("sp", 29);
    registers.insert("fp", 30);
    registers.insert("ra", 31);
}

fn setup_instruction_table(instruction : &mut hash_map::HashMap<&'static str, u32>){
    
    instruction.insert("add", 0x00000020);
    instruction.insert("sub", 0x00000022);
    instruction.insert("addi",0x08000000);
    instruction.insert("sll", 0x00000000);
    instruction.insert("slt", 0x0000002a);
    instruction.insert("and", 0x00000024);
    instruction.insert("or",  0x00000025);
    instruction.insert("nor", 0x00000027);
    instruction.insert("lw",  0x23000000);
    instruction.insert("sw",  0x2b000000);
    instruction.insert("beq", 0x04000000);
    instruction.insert("j",   0x02000000);
    instruction.insert("jr",  0x00000008);
    instruction.insert("nop", 0x00000000);

}