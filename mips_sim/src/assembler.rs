use regex::Captures;
use regex::Regex;
use serde::ser::Error;
use std::collections::hash_map;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

/// An assembler for converting instructions written in the assembly programming language into machine code instructions.
/// The assembler produces a file containing machinecode instructions and a listing file containing additional
/// information about the placement of instructions in memory.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-11-25: First complete version.

const RS_POS: u32 = 21;
const RT_POS: u32 = 16;
const RD_POS: u32 = 11;
const SHAMT: u32 = 6;
const MAX_IMME_SIZE: i32 = u16::MAX as i32; //Maximum value that are used in arithmetic immediate commands
const MAX_BEQ_OFFSET: u32 = i16::MAX as u32; //Maximum value for branch-jumps
const MAX_SHAMT_SIZE: i32 =  31;


/// Enumerates the different types of instructions.

enum InstructionType {
    /// R-Type instruction.
    R1,
    /// First I-Type instruction.
    R2,
    /// First I-Type instruction.
    I1,
    /// Second I-Type instruction.
    I2,
    /// Third I-Type instruction.
    I3,
    /// First J-Type instruction.
    J1,
    /// Second J-Type instruction
    J2,
    /// No opertion instruction.
    N,
    /// Exit Commamd.
    E,
}

/// A struct used keep information about undefined labels.
struct UndefinedLabel {
    /// The row of the file the label appears on.
    file_row: u32,
    /// Index of the address.
    addr_index: u32,
    /// Name of the label.
    label_string: String,
    /// If it is a relative jump or not.
    relative_jump: bool,
}

struct RegexCom {
    r1_type: Regex,
    r2_type: Regex,
    i1_type: Regex,
    i2_type: Regex,
    i3_type: Regex,
    j1_type: Regex,
    j2_type: Regex,
    nop_type: Regex,
    exit_type: Regex,
    com_type: Regex,
    label_type: Regex,
}

/// Parses the input file and returns a tuple containing information for
/// the listing file, machine code file and a hashmap containing all
/// the symbols and corresponding addresses.
///
/// # Arguments
///
/// * `file_path` - The path to the file
///
/// # Returns
///
/// *  (Vec<u32>, Vec<(String, bool)>, hash_map::HashMap<String, u32>)
///
pub fn parse_file(
    file_path: &str,
) -> Option<(
    Vec<u32>,
    Vec<(String, bool)>,
    hash_map::HashMap<String, u32>,
    bool,
    Vec<u32>
)> {
   // Set up all Regexes 
   let regex_coms = RegexCom {
    r1_type: Regex::new(r"^\s(add|sub|nor|or|and|slt)\s+\$(\S+),\s*\$(\S+),\s*\$(\S+)\s*$").unwrap(),
    r2_type: Regex::new(r"^\s(sll|srl|sra)\s+\$(\S+),\s*\$(\S+),\s*(\S+)\s*$").unwrap(),
    i1_type: Regex::new(r"^\s(addi|ori)\s+\$(\S+),\s*\$(\S+),\s*(\S+)\s*$").unwrap(),
    i2_type: Regex::new(r"^\s(beq)\s+\$(\S+),\s*\$(\S+),\s*(\w+)\s*$" ).unwrap(),
    i3_type: Regex::new(r"^\s(lw|sw)\s+\$(\S+),\s*(\S*)\(\$(\S+)\)\s*$").unwrap(), 
    j1_type: Regex::new(r"^\s(j)\s+(\w+)\s*$").unwrap(),
    j2_type: Regex::new(r"^\s(jr)\s+\$(\S+)\s*$").unwrap(),
    nop_type: Regex::new(r"^\s(nop)\s*$").unwrap(),
    exit_type: Regex::new(r"^\s(exit)\s*$").unwrap(),
    com_type: Regex::new("#").unwrap(),
    label_type: Regex::new("([a-z]|[A-z]|[0-9])+[:]").unwrap(),
    };

    let mut registers = hash_map::HashMap::new();
    let mut instructions = hash_map::HashMap::new();

    setup_registers_table(&mut registers);
    setup_instruction_table(&mut instructions);
    // Init table for labels, key to table is a label-string which leads to the address-index (the row) of that label
    let mut labels = hash_map::HashMap::new();

    // Init vector for rows with undefined labels
    let mut undefined_labels: Vec<UndefinedLabel> = Vec::new();
    // Init vector for storing generated machine code & assembler code
    let mut machine_code = Vec::new();
    let mut assembler_code = Vec::new();

    let mut exit_locations: Vec<u32> = Vec::new();

    // Index of machine code address (the row of the machine code innstruction)
    let mut addr_index: u32 = 0;

    // Index of line in file
    let mut file_row: u32 = 0;

    //Bool to remember if file contains errors
    let mut contains_errors : bool = false;

    // File must exist in current path
    if let Ok(lines) = read_lines(file_path) {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mut line) = line {
                // Bool to check if line contains valid code
                let mut contain_code = false;
                if line.len() > 0 {

                    //Check for comments
                    let comment_index = if let Some(index) = locate_comment(&line, &regex_coms) {
                        //Comment found, set comment index to where comment was found
                        index
                    } else {
                        //no comment found, set index to end of line
                        let index = line.len();
                        index
                    };

                    // Locate label on line if it exists.
                    let (label_index, label_found) = if let Some((label, index)) = locate_labels(&line, &regex_coms)
                    {   
                        //Label found, insert current address-index into label-table
                        labels.insert(label, addr_index);
                        //set label index to where comment was found
                        (index, true)
                    } else {
                        //No label found, set label index to beginning of line
                        (0, false)
                    };

                    //Take a slice of the line from end of found label to where a comment was found
                    let line_slice = &line[label_index..comment_index];

                    // If the line contains an identifyable command, assemble the line to machine code and push it to vector

                    if let Some((cap, inst_type)) = identify_type(line_slice, &regex_coms) {

                        if cap.is_some() {
                            let cap = cap.unwrap();

                            //Assemble the line and collect the code in a Result<>
                            let line_code = assemble_line(inst_type, file_row, addr_index, cap, &labels, 
                                    &mut undefined_labels, &registers, &instructions);

                            if let Err(error) = line_code {
                                // If true no need to give error
                                if error.eq("Exit") {
                                    exit_locations.push(file_row);
                                } else {
                                    //ERROR: Something went wrong trying to assemble the line
                                    line.push_str("     <-- Error: ");
                                    line.push_str(error);
                                    contains_errors = true;
                                }
                            } else {
                                contain_code = true;
                                machine_code.push(line_code.unwrap());
                            }
                        } else {
                            //ERROR: No instruction could be captured on the line
                            //Should not be able to get here
                        }

                        addr_index += 1;
                    } else if !label_found && line_slice.len() > 1 {
                        //No label was found and the line is longer than 1 + no instruction was recognized
                        // => Error
                        line.push_str("     <-- Error: instruction not recognized or wrong format on instruction");
                        contains_errors = true;
                    }

                    
                }
                file_row += 1;
                assembler_code.push((line, contain_code));
            }
        }
    } else {
        return None;
    }

    //update commands that referred to previously undefined labels that are now defined
    for undef_label in undefined_labels {
        if let Err(error_row) = fix_undef_label(undef_label, &mut machine_code, &labels) {
            //Something wrong with the called labels!
            let (line, _) = &mut assembler_code[error_row as usize];
            line.push_str("     <-- Error: Label undefined!");
            assembler_code[error_row as usize] = (line.to_string(), true);
            contains_errors = true;
        }
    }

    Some((machine_code, assembler_code, labels, contains_errors, exit_locations))
}

/// Assembles a line using strings for each individual part of the command contained in a regex::Captures
///
/// # Arguments
///
/// * `ìnst_type` - type of instruction that was captured
/// * `file_row` - the current row of the file
/// * `addr_index` - the current row of the machine code
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `labels` - table with found labels and their address-index
/// * `undefined_labels` - Storage for commands that uses labels that have not yet been found
/// * `registers` - A reference to a hashmap containing all registers.
/// * `instruction` - A reference to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_line(
    inst_type: InstructionType,
    file_row: u32,
    addr_index: u32,
    cap: Captures,
    labels: &hash_map::HashMap<String, u32>,
    undefined_labels: &mut Vec<UndefinedLabel>,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
)-> Result<u32, &'static str>{
    let line_code = match inst_type {
        InstructionType::R1 => {
            assemble_r1_type(cap, &registers, &instructions)
        },
        InstructionType::R2 => {
            assemble_r2_type(cap, &registers, &instructions)
        },
        InstructionType::I1 => {
            assemble_i1_type(cap, &registers, &instructions)
        },    
        InstructionType::I2 => { assemble_i2_type(
            file_row,
            addr_index,
            cap,
            labels,
            undefined_labels,
            registers,
            instructions,
        )},
        InstructionType::I3 => {
            assemble_i3_type(cap, &registers, &instructions)
        },
        InstructionType::J1 => assemble_j1_type(
            file_row,
            addr_index,
            cap,
            labels,
            undefined_labels,
            instructions,
        ),
        InstructionType::J2 => {
            assemble_j2_type(cap, registers, instructions)
        }
        InstructionType::N => Ok(0),
        InstructionType::E => handle_exit()
    };

    line_code
}

/// Creates a R-type instruction.  
///
/// # Arguments
///
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `registers` - A reference to a hashmap containing all registers.
/// * `instruction` - A reference to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_r1_type(
    cap: Captures,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rd = &cap[2];
    let rs = &cap[3];
    let rt = &cap[4];

    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(rs, registers)?) << RS_POS;
    instr = instr | (parse_register(rt, registers)?) << RT_POS;
    instr = instr | (parse_register(rd, registers)?) << RD_POS;
    Ok(instr)
}

/// Creates a R-type instruction for shifting.  
///
/// # Arguments
///
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `registers` - A reference to a hashmap containing all registers.
/// * `instruction` - A reference to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_r2_type(
    cap: Captures,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rd = &cap[2];
    let rt = &cap[3];
    let mask = 0x0000FFFF;

    let shamt = (&cap[4]).parse::<i32>();
    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(rt, registers)?) << RT_POS;
    instr = instr | (parse_register(rd, registers)?) << RD_POS;

    let shamt_val = if shamt.is_err() {
        // ErrorVALUE
        Err("the immediate value is not a number")
    } else {
        let shamt_unwrap = shamt.unwrap();
        if shamt_unwrap > MAX_SHAMT_SIZE || shamt_unwrap < 0 {
            //error
            Err("the immediate value is not between 0-31")
        } else {
            Ok((shamt_unwrap & mask) as u32)
        }
    };

    // Check if immi value created error if so return error
    if shamt_val.is_err() {
        return shamt_val;
    } else {
        instr = instr | (shamt_val.unwrap() << SHAMT);
        Ok(instr)
    }
}

/// Creates a first kind I-type instruction.  
///
/// # Arguments
///
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `registers` - A refrence to a hashmap containing all registers.
/// * `instruction` - A refrence to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_i1_type(
    cap: Captures,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rt = &cap[2];
    let rs = &cap[3];
    let mask = 0x0000FFFF;

    let imme = (&cap[4]).parse::<i32>();
    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(rs, registers)?) << RS_POS;
    instr = instr | (parse_register(rt, registers)?) << RT_POS;

    let imme_val = if imme.is_err() {
        // ErrorVALUE
        Err("the immediate value is not a number")
    } else {
        let imme_unwrap = imme.unwrap();
        if imme_unwrap > MAX_IMME_SIZE {
            //error
            Err("the immediate value is too big")
        } else {
            Ok((imme_unwrap & mask) as u32)
        }
    };

    // Check if immi value created error if so return error
    if imme_val.is_err() {
        return imme_val;
    } else {
        instr = instr | imme_val.unwrap();
        Ok(instr)
    }
} 


/// Creates a second kind I-type instruction.  
///
/// # Arguments
///
/// * `file_row` - the current row of the file
/// * `addr_index` - the current row of the machine code
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `labels` - table with found labels and their address-index
/// * `undefined_labels` - Storage for commands that uses labels that have not yet been found
/// * `registers` - A reference to a hashmap containing all registers.
/// * `instruction` - A reference to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_i2_type(
    file_row: u32,
    addr_index: u32,
    cap: Captures,
    labels: &hash_map::HashMap<String, u32>,
    undefined_labels: &mut Vec<UndefinedLabel>,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rt = &cap[3];
    let rs = &cap[2];
    let label = &cap[4];
    let mask = 0x0000FFFF;
    let label_addr: u32;

    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(rs, registers)?) << RS_POS;
    instr = instr | (parse_register(rt, registers)?) << RT_POS;

    if let Some(dest) = labels.get(label) {
        label_addr = *dest;
    } else {
        let temp = UndefinedLabel {
            file_row: file_row,
            addr_index: addr_index,
            label_string: (label).to_string(),
            relative_jump: true,
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
    offset = (!offset) & mask;

    return Ok(instr | offset);
}

/// Creates a second kind I-type instruction.  
///
/// # Arguments
///
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `registers` - A refrence to a hashmap containing all registers.
/// * `instruction` - A refrence to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_i3_type(
    cap: Captures,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let rt = &cap[2];
    let rs = &cap[4];
    let offset = (&cap[3]).parse::<i32>();
    let mask = 0x0000FFFF;

    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(rs, registers)?) << RS_POS;
    instr = instr | (parse_register(rt, registers)?) << RT_POS;

    let offset_val = if offset.is_err() {
        // Error
        Err("offset is to big or not a number")
    } else {
        let offset_unwrap = offset.unwrap();
        if offset_unwrap > MAX_IMME_SIZE {
            // Error
            Err("offset is too big")
        } else {
            Ok((offset_unwrap & mask) as u32)
        }
    };

    if offset_val.is_err() {
        return offset_val;
    } else {
        instr = instr | offset_val.unwrap();
        return Ok(instr);
    }
}

/// Creates a first kind J-type instruction.  
///
/// # Arguments
///
/// * `file_row` - the current row of the file
/// * `addr_index` - the current row of the machine code
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `labels` - table with found labels and their address-index
/// * `undefined_labels` - Storage for commands that uses labels that have not yet been found
/// * `instruction` - A reference to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_j1_type(
    file_row: u32,
    addr_index: u32,
    cap: Captures,
    labels: &hash_map::HashMap<String, u32>,
    undefined_labels: &mut Vec<UndefinedLabel>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let label = &cap[2];
    let mut label_addr: u32 = 0;

    // Check if label is none
    if label == "" {
        return Err("Add label to jump instruction");
    }

    let instr = instructions.get(&cmnd).unwrap();

    if let Some(&dest) = labels.get(label) {
        label_addr = dest;
    } else {
        let temp = UndefinedLabel {
            file_row: file_row,
            addr_index: addr_index,
            label_string: label.to_string(),
            relative_jump: false,
        };
        undefined_labels.push(temp);
    };
    Ok(instr | label_addr)
}

/// Creates a second kind J-type instruction.  
///
/// # Arguments
///
/// * `cap` - A regex::Captures, contains the individual part of the command in assembly code
/// * `registers` - A refrence to a hashmap containing all registers.
/// * `instruction` - A refrence to a hashmap containing all instructions.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the converted instruction.
///
fn assemble_j2_type(
    cap: Captures,
    registers: &hash_map::HashMap<&'static str, u32>,
    instructions: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    let cmnd = &cap[1];
    let dest = &cap[2];

    let mut instr = *instructions.get(&cmnd).unwrap();

    instr = instr | (parse_register(dest, registers)?) << RS_POS;
    Ok(instr)
}

fn handle_exit(
) -> Result<u32, &'static str> {
    Err("Exit")
}

/// Returns code for register as u32, if register is invalid, returns Err.
///
/// # Arguments
///
/// * `reg_cap` - The register to be checked.
/// * `registers` - A refrence to a hashmap containing all registers.
///
/// # Returns
///
/// *  Result<u32, &'static str> - Where the u32 is the machine code for the register.
///
fn parse_register(
    reg_cap: &str,
    registers: &hash_map::HashMap<&'static str, u32>,
) -> Result<u32, &'static str> {
    // Check if register can be found in register table
    let reg = if let Some(&r) = registers.get(&reg_cap) {
        Ok(r)
    } else {
        // Register not found in table, use data as register number
        let r = reg_cap.parse::<u32>();
        if r.is_err() {
            // Error
            Err("register does not exist")
        } else {
            let r_value = r.unwrap();
            if r_value > 31 {
                //Invalid register
                Err("register should be between 0-31")
            } else {
                Ok(r_value)
            }
        }
    };

    reg
}

/// Identifies the type of a given instruction with regex, and returns it's type and the regex string associated with it.
/// If type could not be identified, returns none.
///
/// # Arguments
///
/// * `text` - A line of the input file.
///
/// # Returns
///
/// *  Option<(String, InstructionType) - Where the String is the regex string and the InstructionType is the type.
///
fn identify_type<'a>(text: &'a str, regex: &RegexCom) -> Option<(Option<Captures<'a>>, InstructionType)> {

    if regex.r1_type.is_match(text) {
        let cap = regex.r1_type.captures(text);
        return Some((cap, InstructionType::R1));
    } if regex.r2_type.is_match(text) {
            let cap = regex.r2_type.captures(text);
            return Some((cap, InstructionType::R2));
    } else if regex.i1_type.is_match(text) {
        let cap = regex.i1_type.captures(text);
        return Some((cap, InstructionType::I1));
    } else if regex.i2_type.is_match(text) {
        let cap = regex.i2_type.captures(text);
        return Some((cap, InstructionType::I2));
    } else if regex.i3_type.is_match(text) {
        let cap = regex.i3_type.captures(text);
        return Some((cap,InstructionType::I3 ));
    } else if regex.j1_type.is_match(text) {
        let cap = regex.j1_type.captures(text);
        return Some((cap, InstructionType::J1));
    } else if regex.j2_type.is_match(text) {
        let cap = regex.j2_type.captures(text);
        return Some((cap, InstructionType::J2));
    } else if regex.nop_type.is_match(text) {
        let cap = regex.nop_type.captures(text);
        return Some((cap, InstructionType::N));
    } else if regex.exit_type.is_match(text) {
        let cap = regex.exit_type.captures(text);
        return Some((cap, InstructionType::E));
    } else {
        return None;
    }
}

/// Returns an Iterator to the Reader of the lines of the file.
/// The output is wrapped in a Result to allow matching on errors.
///
/// # Arguments
///
/// * `filename` - Name of the file.
///
/// # Returns
///
/// *  io::Result<io::Lines<io::BufReader<File>>> - an Iterator to the Reader of the lines of the file.
///
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Locates a comment in a line. Returns the offset for of the first comment found wrapped in an Option.
/// If no comment could be found, returns None.
///
/// # Arguments
///
/// * `line` - Line to be checked for comments.
///
/// # Returns
///
/// *  Option<usize> - where usize is the offset.
///
fn locate_comment(line: &str, regex: &RegexCom) -> Option<usize> {
    // Will itterate over "line" string and search for "#"
    // Will return first "#" found, if no "#" is found will return empty error.
    if !line.contains("#") {
        return None;
    }
    for cap in regex.com_type.find_iter(line) {
        return Some(cap.start());
    }
    return None;
}

/// Locates a label in a line. Returns the end-index for a found label and the label itself wrapped in an Option.
/// If no label could be found, returns None.
///
/// # Arguments
///
/// * `line` - Line to be checked for labels.
///
/// # Returns
///
/// *  Option<(String,usize)> - where usize is the end-index for the label and String is the label found.
/// 
///
fn locate_labels(line: &str, regex: &RegexCom) -> Option<(String, usize)> {
    for cap in regex.label_type.find_iter(line)
    {
        if cap.start() == 0 {
            return Some((line[0..cap.end() - 1].to_string(), cap.end()));
        }
    }
    return None;
}

/// Fixes machine-code that addresses previously undefined labels that are now defined.
///
/// # Arguments
///
/// * `undefined_labels` - Vector containing all undefined labels.
/// * `machine_code` - Vector containing the converted machine code.
/// * `labels` - Hashmap containing all the labels.
///
/// # Returns
///
/// *  Result<(),u32>
///
fn fix_undef_label(
    undefined_label: UndefinedLabel,
    machine_code: &mut Vec<u32>,
    labels: &hash_map::HashMap<String, u32>,
) -> Result<(), u32> {
    //Destruct fields in undef_label
    let UndefinedLabel {
        file_row,
        addr_index,
        label_string: label,
        relative_jump: rel_jump,
    } = undefined_label;
    let row_index = addr_index as usize;

    if !labels.contains_key(&label) {
        //Label is still undefined! indicate error
        return Err(file_row);
    } else if !rel_jump {
        //Command indicates absolute jump, insert jump into machine-code
        machine_code[row_index] = machine_code[row_index] | labels.get(&label).unwrap();
    } else {
        //Command indicates relative jump
        let label_addr = labels.get(&label).unwrap();

        // Calculate the relative jump
        let offset = label_addr - addr_index - 1;// Minus one as the pc counter will jump one before excecuting instruction

        // Check if relative jump is to far away
        if offset > MAX_BEQ_OFFSET {
            return Err(file_row);
        }
        // insert jump into machine-code
        machine_code[row_index] = machine_code[row_index] | offset;
        
    }
    Ok(())
}

/// Initilizes the register table which contains all allowed registers.
///
/// # Arguments
///
/// * `registers` - The register table.
///
fn setup_registers_table(registers: &mut hash_map::HashMap<&'static str, u32>) {
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

/// Initilizes the instructions table which contains all allowed instructions.
///
/// # Arguments
///
/// * `instructions` - The instruction table.
///
fn setup_instruction_table(instruction: &mut hash_map::HashMap<&str, u32>) {
    instruction.insert("add", 0x00000020);
    instruction.insert("sub", 0x00000022);
    instruction.insert("addi", 0x08000000 << 2);
    instruction.insert("sll", 0x00000000);
    instruction.insert("slt", 0x0000002a);
    instruction.insert("and", 0x00000024);
    instruction.insert("or", 0x00000025);
    instruction.insert("nor", 0x00000027);
    instruction.insert("lw", 0x23000000 << 2);
    instruction.insert("sw", 0x2b000000 << 2);
    instruction.insert("beq", 0x04000000 << 2);
    instruction.insert("j", 0x02000000 << 2);
    instruction.insert("jr", 0x00000008);
    instruction.insert("nop", 0x00000000);
    instruction.insert("ori", 0x0d000000<<2);
    instruction.insert("srl", 0x00000002);
    instruction.insert("sra", 0x00000003);
}
