use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::env;
use std::collections::hash_map;

fn main() { 
    let mut listing_file = Vec::new();
    let mut asm_file = Vec::new();

    listing_file.push(("# This is an example".to_string(), false));
    listing_file.push(("labl0:".to_string(), false));
    listing_file.push(("label24: addi  $t1, $zero, 1   # A comment".to_string(), true));
    listing_file.push(("        addi  $t2, $zero, 2".to_string(), true));
    listing_file.push(("        addi  $t3, $zero, 3".to_string(), true));
    listing_file.push(("        addi  $t4, $zero, -4".to_string(), true));

    asm_file.push(0x20090001);
    asm_file.push(0x200a0002);
    asm_file.push(0x200b0003);
    asm_file.push(0x200cfffc);
    asm_file.push(0x20090001);

    write_files(asm_file, listing_file);
}

fn write_files(machine_code: Vec<u32>, assembler_code: Vec<(String, bool)>, symbol_table::HashMap<String, u32>) {
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

    for (label, addr) in &symbol_table {
        write!(&mut machine_writer, "{:#010x}\n", machine_code[i]);
    }
    
}