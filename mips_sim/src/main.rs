#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use mips_sim::*;
use std::{thread::{self, sleep}, sync::{Arc, Mutex}};

mod units;
mod assembler;

use crate::units::program_counter::*;
use crate::units::instruction_memory::*;
use crate::units::add_unit::*;
use crate::units::unit::*;
use crate::units::control::*;
use bitvec::prelude::*;
use assembler::parse_file;
use std::convert::AsRef;


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
    let (machine_code, assembler_code, labels, registers) = parse_file(file_path);


    // Add vector with machine-code to a vector of Words
    let mut instructions: Vec<Word> = Vec::new();
    for instruction in machine_code{
        instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
    }
    println!("Have at memory 0: {}",instructions[0]);

    //Create empty objects for testing
    let mut empty_control : Box<dyn  Unit> =  Box::new(EmptyUnit::new("control"));
    let mut empty_alu: Box<dyn  Unit> =  Box::new(EmptyUnit::new("alu"));
    let mut empty_add: Box<dyn  Unit> =  Box::new(EmptyUnit::new("add"));
    let mut empty_alu_ctrl: Box<dyn  Unit> = Box::new(EmptyUnit::new("alu-control"));
    let mut empty_dm: Box<dyn  Unit> =  Box::new(EmptyUnit::new("data-memory"));
    let mut empty_im: Box<dyn  Unit> =  Box::new(EmptyUnit::new("instruction-memory"));
    
    let mut empty_mux_branch: Box<dyn  Unit> = Box::new(EmptyUnit::new("mux-branch"));
    let mut  empty_mux_regdst: Box<dyn  Unit> =  Box::new(EmptyUnit::new("mux_Regdst"));
    let mut  empty_mux_jump: Box<dyn  Unit> =  Box::new(EmptyUnit::new("mux-jump"));
    let mut  empty_mux_alusrc: Box<dyn  Unit> =  Box::new(EmptyUnit::new("mux-alusrc")) ;
    let mut empty_mux_memtoreg: Box<dyn  Unit> =  Box::new(EmptyUnit::new("mux-memtoreg"));
    let mut  empty_mux_jr: Box<dyn  Unit> =  Box::new(EmptyUnit::new("mux-memtoreg"));

    let mut  empty_pc: Box<dyn  Unit> =  Box::new(EmptyUnit::new("pc"));
    let mut  empty_reg: Box<dyn  Unit> =  Box::new(EmptyUnit::new("register-file"));
    let mut  empty_se: Box<dyn  Unit> =  Box::new(EmptyUnit::new("sign-extender"));
    let mut  empty_conc: Box<dyn  Unit> =  Box::new(EmptyUnit::new("concater"));
    let mut  empty_ander: Box<dyn  Unit> =  Box::new(EmptyUnit::new("ander"));

    //Create mutexes of Box<empty units>
    let  mut_empty_control =  Mutex::new(&mut empty_control);
    let  mut_empty_alu =  Mutex::new(&mut empty_alu);
    let  mut_empty_add =  Mutex::new(&mut empty_add);
    let  mut_empty_alu_ctrl =  Mutex::new(&mut empty_alu_ctrl);
    let  mut_empty_dm =  Mutex::new(&mut empty_dm);
    let  mut_empty_im =  Mutex::new(&mut empty_im);
    
    let  mut_empty_mux_branch = Mutex::new(&mut empty_mux_branch);
    let  mut_empty_mux_regdst =  Mutex::new(&mut empty_mux_regdst);
    let  mut_empty_mux_jump =  Mutex::new(&mut empty_mux_jump);
    let  mut_empty_mux_alusrc =  Mutex::new(&mut empty_mux_alusrc) ;
    let  mut_empty_mux_memtoreg =  Mutex::new(&mut empty_mux_memtoreg);
    let  mut_empty_mux_jr =  Mutex::new(&mut empty_mux_jr);

    let mut_empty_pc =  Mutex::new(&mut empty_pc);
    let mut_empty_reg =  Mutex::new(&mut empty_reg );
    let mut_empty_se=  Mutex::new(&mut empty_se);
    let mut_empty_conc =  Mutex::new(&mut empty_conc);
    let mut_empty_ander =  Mutex::new(&mut empty_ander);

    // Create all objects
    let mut pc: Box<dyn  Unit>  = Box::new(ProgramCounter::new());
    let mut instr_mem: Box<dyn  Unit> = Box::new(InstructionMemory::new(instructions));
    let mut sign_extend: Box<dyn  Unit> = Box::new(SignExtend::new());
    let mut alu_add: Box<dyn  Unit> = Box::new(AddUnit::new());
    

    // Create mutexes for "real" objects
    let mutex_pc = Mutex::new(&mut pc);
    let mutex_instr_mem = Mutex::new(&mut instr_mem);
    let mutex_sign_extend = Mutex::new(&mut sign_extend);
    let mutex_alu_add = Mutex::new(&mut alu_add);
    
    
    // Assemble Controller
    let mut control: Control<'static> = Control::new(&mut_empty_mux_regdst,
            &mut_empty_mux_jump, 
            &mut_empty_mux_jr, 
            &mut_empty_ander, 
            &mut_empty_mux_alusrc,
            &mut_empty_mux_memtoreg,
            &mut_empty_alu_ctrl,
            &mut_empty_reg,
            &mut_empty_dm);

    // Add components to connect with program counter
    (pc as Box<InstructionMemory>).as_mut().set_instr_memory(&mutex_instr_mem); 
    pc.set_concater(&mut_empty_conc);
    pc.set_add(&mutex_alu_add);
    pc.set_mux_branch(&mut_empty_mux_branch);

    // Add components to connect with instruction memory
    instr_mem.set_aluctrl(&mut_empty_mux_alusrc);
    instr_mem.set_concater(&mut_empty_conc);
    instr_mem.set_control(& mut_empty_control);
    instr_mem.set_reg(& mut_empty_reg);
    instr_mem.set_signextend(&mutex_sign_extend);

    // Add components to connect with sign_extend
    //sign_extend.set_add(&mut alu_add);

    // Add components to connect with ALU ADD
    //alu_add.set_mux_branch(&mut empty_mux);

    pc.execute();
    instr_mem.execute();
    
    sign_extend.execute();
    alu_add.execute();
    
 /*
    let pc_arc = Arc::new(Mutex::new(pc));
    let pc_ref = Arc::clone(&pc_arc);

    let im_arc = Arc::new(Mutex::new(instr_mem));
    let im_ref = Arc::clone(&im_arc);

    // Thread for the program counter
    let regfile_thread = thread::spawn(move||{
        let mut reg_file = im_ref.lock().unwrap();
        loop {
            reg_file.execute();
        }
    });
    
    
    // Thread for the instruction memory
    let pc_thread = thread::spawn(move||{
        let mut prog_c = pc_ref.lock().unwrap();
        
            prog_c.execute();
        
    }); */
    
   // let instr_mem = Arc::new(Mutex::new(instr_mem));
   // let instr_mem_ref = Arc::clone(&instr_mem);
}
/* 
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::Vec2;
use mips_sim::*;
mod assembler;

use assembler::parse_file;
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
        min_window_size: Option::from(Vec2::new(1300 as f32, 500 as f32)),
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: false,
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

    let file_path = "test1";
    let (machine_code, assembler_code, labels, registers) = parse_file(file_path);

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            Box::new(MipsApp::new(
                cc,
                labels,
                registers,
                machine_code,
                assembler_code,
            ))
        }),
    );
}*/
