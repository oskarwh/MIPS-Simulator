/* 
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use mips_sim::*;
use std::{thread::{self, sleep}, sync::{Arc, Mutex}};

mod units;
mod assembler;

use crate::units::program_counter::*;
use crate::units::instruction_memory::*;
use crate::units::unit::*;
use bitvec::prelude::*;
use assembler::parse_file;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use bitvec::view::BitView;

    use crate::units::sign_extend::{self, SignExtend};
    /*tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc))),
    );*/

    //Parse the file into machine code
    let file_path = "test1";
    let (machine_code, assembler_code, labels) = parse_file(file_path);


    // Add vector with machine-code to a vector of Words
    let mut instructions: Vec<Word> = Vec::new();
    for instruction in machine_code{
        instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
    }
    println!("Have at memory 0: {}",instructions[0]);

    //Create empty objects for testing
    let mut empty_control = EmptyUnit::new("control");
    let mut empty_alu = EmptyUnit::new("alu");
    let mut empty_add = EmptyUnit::new("add");
    let mut empty_alu_ctrl = EmptyUnit::new("alu-control");
    let mut empty_dm = EmptyUnit::new("data-memory");
    let mut empty_im = EmptyUnit::new("instruction-memory");
    let mut empty_mux = EmptyUnit::new("mux");
    let mut empty_pc = EmptyUnit::new("pc");
    let mut empty_reg = EmptyUnit::new("register-file");
    let mut empty_se= EmptyUnit::new("sign-extender");
    let mut empty_conc = EmptyUnit::new("concater");

    // Create all objects
    let mut pc: ProgramCounter<'static>  = ProgramCounter::new();
    let mut instr_mem: InstructionMemory<'static> = InstructionMemory::new(instructions);
    let mut sign_extend: SignExtend<'static> = SignExtend::new();
    
 
    
    // Add components to connect with program counter
    pc.set_instr_memory(&mut instr_mem);
    pc.set_concater(&mut empty_conc);
    pc.set_add(&mut empty_add);
    pc.set_mux_branch(&mut empty_mux);

    // Add components to connect with instruction memory
    instr_mem.set_aluctrl(&mut empty_alu_ctrl);
    instr_mem.set_concater(&mut empty_conc);
    instr_mem.set_control(&mut empty_control);
    instr_mem.set_reg(&mut empty_reg);
    instr_mem.set_signextend(&mut sign_extend);

    // Add components to connect with sign_extend
    sign_extend.set_add(&mut empty_add);

    pc.execute();
    instr_mem.execute();
    sign_extend.execute();
    
    

    let pc_arc = Arc::new(Mutex::new(pc));
    let pc_ref = Arc::clone(&pc_arc);

    let im_arc = Arc::new(Mutex::new(instr_mem));
    let im_ref = Arc::clone(&im_arc);

    // Thread for the program counter
    let pc_thread = thread::spawn(move||{
        im_ref.execute();
    });

    let instr_mem = Arc::new(Mutex::new(instr_mem));
    let instr_mem_ref = Arc::clone(&instr_mem);

    // Thread for the instruction memory
    let instr_mem_thread = thread::spawn(move||{
        pc_ref.execute();
    });
}
*/
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::Vec2;
use mips_sim::*;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: Option::from(Vec2::new(1000 as f32, 400 as f32)),
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc))),
    );
}