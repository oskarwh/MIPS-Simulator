#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use mips_sim::*;
use std::{thread::{self, sleep}, sync::{Arc, Mutex}};

#[path = "units/unit.rs"]mod unit;
#[path = "units/program_counter.rs"]mod program_counter;
#[path = "units/add_unit.rs"]mod add_unit;
#[path = "units/alu.rs"]mod alu;
#[path = "units/concater.rs"]mod concater;
#[path = "units/data_memory.rs"]mod data_memory;
#[path = "units/instruction_memory.rs"]mod instruction_memory;
#[path = "units/mux.rs"]mod mux;
#[path = "units/registers.rs"]mod registers;
#[path = "units/sign_extend.rs"]mod sign_extend;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc))),
    );

    // Create all objects
    let pc: Unit = program_counter.new();
    // Add file with instructions
    let instr_mem: Unit = instruction_memory.new();


    // Add components to connect with program counter
    pc.set_instr_memory(&instr_mem);

    // Add components to connect with instruction memory
    instr_mem.set_pc(&pc);



    let pc = Arc::new(Mutex::new(pc));
    let pc_ref = Arc::clone(&pc);

    // Thread for the program counter
    let pc_thread = thread::spawn(move||{
        instr_mem_ref.execute();
    });

    let instr_mem = Arc::new(Mutex::new(instr_mem));
    let instr_mem_ref = Arc::clone(&instr_mem);

    // Thread for the instruction memory
    let instr_mem_thread = thread::spawn(move||{
        pc_ref.execute();
    });
}
/* 
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use mips_sim::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc))),
    );
}
*/