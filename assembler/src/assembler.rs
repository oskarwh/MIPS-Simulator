extern crate regex;

use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::env;
use regex::Regex;
use regex::Captures;
use std::collections::hash_map;

const RS_POS: u32 = 21;
const RT_POS: u32 = 16;
const RD_POS: u32 = 11;
const MAX_IMME_SIZE: u32 = u16::MAX as u32;
const MAX_BEQ_OFFSET: u32 = i16::MAX as u32;

enum ErrorType {
    BadString,
    HarmlessErr,
}


enum InstructionType{
    R,
    I1,
    I2,
    I3,
    J1,
    J2,
    N,
}

struct UndefinedLabel{
    file_row :u32,
    addr_index: u32,
    label_string: String,
    relative_jump : bool
}


fn main() {

    let args: Vec<String> = env::args().collect();

    
    //Check that the program has the right amount of arguments
    if args.len() < 2 || args.len() > 2{
        panic!("Usage: ./assembler filename\n");
    }
    let file_path = &args[1]; 

    let (machine_code, assembler_code, labels) = parse_file(file_path);
    write_files(machine_code, assembler_code, labels);
}


fn parse_file(file_path : &str) -> (Vec<u32>, Vec<(String, bool)>, hash_map::HashMap<String, u32>) {

    let mut registers = hash_map::HashMap::new();
    let mut instructions = hash_map::HashMap::new();
    setup_registers_table(&mut registers);
    setup_instruction_table(&mut instructions);
    // Init table for labels
    let mut labels = hash_map::HashMap::new();

    // Init vector for rows with undefined labels
    let mut undefined_labels :Vec<UndefinedLabel>= Vec::new();
    // Init vector for storing generated machine code & assembler code
    let mut machine_code = Vec::new();
    let mut assembler_code = Vec::new();


    // Index of machine code address
    let mut addr_index :u32 = 0;

    // Index of line in file
    let mut file_row :u32 = 0;



    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mut line) = line{
                // Bool to check if line contains valid code
                let mut contain_code = false;
                if line.len() > 0 {
                    
                    //Check for comments
                    let comment_index = if let Ok(index) = locate_comment(&line){
                        //Comment found, set comment index to where comment was found
                        index
                    }
                    else{
                        //no comment found, set index to end of line
                        let index= line.len();
                        index
                    };

                    // Locate label on line if it exists.
                    let mut label_index :usize = 0;
                    let mut label_found = false;
                    if let Ok((label, i)) = locate_labels(&line) {
                        labels.insert(label, addr_index);   
                        label_index = i;    
                        label_found = true;       
                    }

                    //Take a slice of the line from start to where a comment was found
                    let line_slice = &line[..comment];
                    
                    // If the line contains an identifyable command, assemble the line to machine code and push it to vector
                    if let Some((regex, inst_type)) = identify_type(line_slice){
                        let cap = capture_command(line_slice, &regex);
                        if cap.is_some(){
                            let cap = cap.unwrap();
                            let line_code = match inst_type {
                                InstructionType::R => assemble_r_type(cap, &registers, &instructions),
                                InstructionType::I1 => assemble_i1_type(cap, &registers, &instructions),
                                InstructionType::I2 => assemble_i2_type(file_row,addr_index, cap, &labels, &mut undefined_labels, &registers, &instructions),
                                InstructionType::I3 => assemble_i3_type(cap, &registers, &instructions),
                                InstructionType::J1 => assemble_j1_type(file_row,addr_index, cap, &labels, &mut undefined_labels, &instructions),
                                InstructionType::J2 => assemble_j2_type(cap, &registers, &instructions),
                                InstructionType::N => Ok(0)
                            };
    
                            if let Err(error) = line_code{
                                //HANDLE ERROR ON LINE         
                                line.push_str("     Error: ");
                                line.push_str(error);
                                contain_code = false; 
                            }else{                    
                                machine_code.push(line_code.unwrap()); 
                            }
                        }else{
                            //HANDLE ERROR ON LINE         
                            line.push_str("     Error: wrong format on instruction");
                            contain_code = false; 
                        }
                        
                        contain_code = true;
                        addr_index += 1;
                    }else if label_found || line_slice.len() < 1{
                        contain_code = false; 
                    }else{
                        line.push_str("     Error: instruction not recognized");
                        contain_code = false; 
                    }

                      
                    file_row +=1;
                }
                assembler_code.push((line, contain_code)); 
            }
    }
    }else{
        panic!();
    }
    
    if let Err(error_row) = fix_undef_labels(undefined_labels, &mut machine_code, &labels) {
        //Something wrong with the called labels!
        let (line, bool) = &mut assembler_code[error_row as usize];
        line.push_str("     Error: Label undefined!");
        assembler_code[error_row as usize] = (line.to_string(), true ); 
    }

    (machine_code, assembler_code, labels) 
}

fn assemble_r_type(cap:Captures, registers : &hash_map::HashMap<&'static str ,u32>, instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str>{
    let cmnd = &cap[1];
    let rd = &cap[2];
    let rs = &cap[3];
    let rt = &cap[4];
    
    let mut instr = *instructions.get(&cmnd).unwrap();


    instr = instr |  (parse_register(rs, registers)?) << RS_POS;
    instr = instr |  (parse_register(rt, registers)?) << RT_POS;
    instr = instr |  (parse_register(rd, registers)?) << RD_POS;
    Ok(instr)
}


fn assemble_i1_type(cap:Captures, registers : &hash_map::HashMap<&'static str ,u32>, instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str>{
    let cmnd = &cap[1];
    let rt = &cap[2];
    let rs = &cap[3];
  
    let imme = (&cap[4]).parse::<u32>();
    
    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr |  (parse_register(rs, registers)?) << RS_POS;
    instr = instr |  (parse_register(rt, registers)?) << RT_POS;

    let imme_val =  if imme.is_err() {
         // ErrorVALUE
         Err("the immediate value is not a number")
    }else{
        let imme_unwrap = imme.unwrap();
        if imme_unwrap > MAX_IMME_SIZE{
            //error
            Err("the immediate value is too big")
        }else{
            Ok(imme_unwrap)
        }
    };

    // Check if immi value created error if so return error
    if imme_val.is_err() {
        return imme_val;
    }else {
        instr = instr | imme_val.unwrap();
        Ok(instr)
    }   
}

fn assemble_i2_type(file_row:u32,addr_index:u32, cap:Captures, labels: &hash_map::HashMap<String, u32>, undefined_labels: &mut Vec<UndefinedLabel>,  registers : &hash_map::HashMap<&'static str ,u32>,instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str>{
    let cmnd = &cap[1];
    let rt = &cap[3];
    let rs = &cap[2];
    let label = &cap[4];
    let mask = 0x0000FFFF;
    let mut label_addr: u32= 0;
    
    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr |  (parse_register(rs, registers)?) << RS_POS;
    instr = instr |  (parse_register(rt, registers)?) << RT_POS;

    if let Some(dest) = labels.get(label) {
        label_addr = (*dest);
    }else {
        let temp = UndefinedLabel {
            file_row:file_row,
            addr_index: addr_index,
            label_string: (label).to_string(),
            relative_jump:true
        };
        undefined_labels.push(temp);
        return Ok(instr);
    };


    let mut offset: u32;
    // Calculate the relative jump
    offset = addr_index - label_addr;
   
    // Check if relative jump is to far away
    if offset > MAX_BEQ_OFFSET {
        return Err("relative jump is too big");
    }

    // Otherwise negate the relative jump as it will be behind us
    offset = ((!offset)) & mask;
    
    return Ok(instr | offset);
}


fn assemble_i3_type(cap:Captures, registers : &hash_map::HashMap<&'static str ,u32>,instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rt = &cap[2];
    let rs = &cap[4];
    let offset = (&cap[3]).parse::<u32>();
    
    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr |  (parse_register(rs, registers)?) << RS_POS;
    instr = instr |  (parse_register(rt, registers)?) << RT_POS;

    let offset_val = if offset.is_err() {
        // Error
        Err("offset is not a number")
    }else {
        let offset_unwrap = offset.unwrap();
        if offset_unwrap > MAX_IMME_SIZE{
            // Error
            Err("offset is too big")
        }else{
            Ok(offset_unwrap)
        }
    };
    
    if offset_val.is_err() {
        return offset_val;
    }else {
        instr = instr | offset_val.unwrap();
        return Ok(instr);
    }  
}





fn assemble_j1_type(file_row:u32,addr_index:u32, cap:Captures, labels: &hash_map::HashMap<String, u32>, undefined_labels: &mut Vec<UndefinedLabel>, instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str>{
    let cmnd = &cap[1];
    let label = &cap[2];
    let mut label_addr: u32 = 0;

    let instr = instructions.get(&cmnd).unwrap();

    if let Some(&dest) = labels.get(label) {
        label_addr = dest;
    }else {
        let temp = UndefinedLabel {
            file_row: file_row,
            addr_index: addr_index,
            label_string: label.to_string(),
            relative_jump:false
        };
        undefined_labels.push(temp);
    };
    Ok(instr | label_addr)
} 

fn assemble_j2_type(cap:Captures, registers : &hash_map::HashMap<&'static str ,u32>, instructions : &hash_map::HashMap<&'static str ,u32>) -> Result<u32, &'static str>{
    let cmnd = &cap[1];
    let dest = &cap[2];

    let mut instr = *instructions.get(&cmnd).unwrap();
   
    instr = instr | (parse_register(dest, registers)?);
    Ok(instr)
} 

/**
 * Returns code for register as u32, if register is invalid, returns None
 */
fn parse_register(reg_cap:&str, registers : &hash_map::HashMap<&'static str ,u32>)->Result<u32, &'static str>{
    // Check if register can be found in register table
   let reg = if let Some(&r) = registers.get(&reg_cap){ 
       Ok(r)
   } else {
       // Register not found in table, use data as register number
       let r = reg_cap.parse::<u32>();
       if r.is_err() {
            // Error
            Err("register does not exist")
       }else {
            let r_value = r.unwrap();
            if r_value > 31{
                //Invalid register
                Err("register should be between 0-31")
            }else{
                Ok(r_value)
            }
       }
   };

   reg
}



fn identify_type(text: &str)->Option<(String, InstructionType)>{

    let r_type = Regex::new(r"(add|sub|nor|or|and|slt)").unwrap();
    let i1_type = Regex::new(r"(addi)").unwrap();
    let i2_type = Regex::new(r"(beq)").unwrap();
    let i3_type = Regex::new(r"(lw|sw)").unwrap();
    let j1_type = Regex::new(r"(j)").unwrap();
    let j2_type = Regex::new(r"(jr)").unwrap();
    let nop_type = Regex::new(r"(nop)").unwrap();

    if r_type.is_match(text) {
        return Some((r"(add|sub|nor|or|and|slt)\s+\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra),\s*\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra),\s*\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra)".to_string(), InstructionType::R));
    }
    else if i1_type.is_match(text){
        return Some((r"(addi)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]+|[0-9]|zero|at),\s*-*[0-9]+".to_string(), InstructionType::I1));
    }
    else if i2_type.is_match(text){
        return Some((r"(beq)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*\$([avtsk][0-9]+|[0-9]|zero|at),\s*(\w+)".to_string(), InstructionType::I2));
    }
    else if i3_type.is_match(text){
        return Some((r"(lw|sw)\s+\$([avtsk][0-9]|[0-9]+|zero|at),\s*([0-9]*)\(\$([avtsk][0-9])\)".to_string(), InstructionType::I3));
    }
    else if j1_type.is_match(text){
        return Some((r"(j)\s+(\w+)".to_string(),InstructionType::J1));
    }else if j2_type.is_match(text){
        return Some((r"(jr)\s+\$([avtsk][0-9]|[0-9]+|zero|at|gp|sp|fp|ra)".to_string(),InstructionType::J2));
    }else if nop_type.is_match(text) {
        return Some((r"(nop)".to_string(), InstructionType::N));
    }else{
        return None;
    }
}

/**
 * Captures commands from a text
 */
fn capture_command<'a>(text: &'a str, regex:&'a str) -> Option<regex::Captures<'a>>{
    let re = Regex::new(regex).unwrap();
    let captures = re.captures(text);

    captures

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

    for cap in Regex::new("#").unwrap().find_iter(line) {
        return Ok(cap.start());
    }
    return Err(ErrorType::HarmlessErr);
}

fn locate_labels(line: &str) -> Result<(String, usize), ErrorType> {
    for cap in Regex::new("([a-z]|[A-z]|[0-9])+[:]").unwrap().find_iter(line) {
        if cap.start() == 0 {
            //println!("{:?}", cap);
            return Ok((line[0..cap.end()-1].to_string(), cap.end()));
            
        }
    }
    return Err(ErrorType::BadString);
}


fn fix_undef_labels(undefined_labels: Vec<UndefinedLabel>, machine_code: &mut Vec<u32>, labels : &hash_map::HashMap<String, u32>) -> Result<(),u32>{
    for undef_label in undefined_labels{
        //Destruct fields in undef_label
        let UndefinedLabel {file_row: file_row, addr_index: addr_index, label_string: label,relative_jump : rel_jump} = undef_label;
        let row_index = addr_index as usize;
        if !labels.contains_key(&label){
            return Err(file_row);
        }
        else if !rel_jump{

            machine_code[row_index] = machine_code[row_index] | labels.get(&label).unwrap();
        }else{

            let label_addr = labels.get(&label).unwrap();

            // Calculate the relative jump
            let mut offset  =  label_addr - addr_index;

            // Check if relative jump is to far away
            if offset > MAX_BEQ_OFFSET {
                return Err(file_row);
            }
            
            machine_code[row_index] = machine_code[row_index] | offset;
        }
    }
    Ok(())
}


///
/// 
/// 
/// 
/// 
/// 
fn write_files(machine_code: Vec<u32>, assembler_code: Vec<(String, bool)>, symbol_table: hash_map::HashMap<String, u32>) {
    let listing_file = File::create("asm_listing").unwrap();
    let machine_file = File::create("asm_instr").unwrap();
    let mut list_writer = BufWriter::new(&listing_file);
    let mut machine_writer = BufWriter::new(&machine_file);
    let mut i = 0; 
    for assembler_line in assembler_code.iter() {
        // Check if line contains machine code
        if assembler_line.1 {
            // Write to listing file with 
            write!(&mut list_writer, "{:#010x}  {:#010x}  {}\n", i*4, machine_code[i], assembler_line.0);
            write!(&mut machine_writer, "{:#010x}\n", machine_code[i]);
            i+=1; 
        } else {
            write!(&mut list_writer, "{:24}{}\n", "",assembler_line.0);
        }
    }

    write!(&mut list_writer, "\n  {:10}   {:10}\n", "Label name", "Address");
    write!(&mut list_writer, "┌-----------┬------------┐\n");
    for (label, addr) in &symbol_table {
        write!(&mut list_writer, "│{:10} │ {:#010x} │\n", label, addr);
    }
    write!(&mut list_writer, "└-----------┴------------┘\n");

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
    instruction.insert("addi",0x08000000<<2);
    instruction.insert("sll", 0x00000000);
    instruction.insert("slt", 0x0000002a);
    instruction.insert("and", 0x00000024);
    instruction.insert("or",  0x00000025);
    instruction.insert("nor", 0x00000027);
    instruction.insert("lw",  0x23000000<<2);
    instruction.insert("sw",  0x2b000000<<2);
    instruction.insert("beq", 0x04000000<<2);
    instruction.insert("j",   0x02000000<<2);
    instruction.insert("jr",  0x00000008);
    instruction.insert("nop", 0x00000000);

}