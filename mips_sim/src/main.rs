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
use crate::units::alu::*;
use bitvec::prelude::*;
use assembler::parse_file;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use bitvec::view::BitView;

    use crate::units::{sign_extend::{self, SignExtend}, mux::Mux, data_memory::DataMemory, registers::Registers, alu_control::AluControl, ander::Ander};

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
    let mut empty_control =  (EmptyUnit::new("control"));
    let mut empty_alu =  (EmptyUnit::new("alu"));
    let mut empty_add =  (EmptyUnit::new("add"));
    let mut empty_alu_ctrl= (EmptyUnit::new("alu-control"));
    let mut empty_dm =  (EmptyUnit::new("data-memory"));
    let mut empty_im =  (EmptyUnit::new("instruction-memory"));
    
    let mut empty_mux_branch = (EmptyUnit::new("mux-branch"));
    let mut  empty_mux_regdst =  (EmptyUnit::new("mux_Regdst"));
    let mut  empty_mux_jump =  (EmptyUnit::new("mux-jump"));
    let mut  empty_mux_alusrc =  (EmptyUnit::new("mux-alusrc")) ;
    let mut empty_mux_memtoreg =  (EmptyUnit::new("mux-memtoreg"));
    let mut  empty_mux_jr =  (EmptyUnit::new("mux-memtoreg"));

    let mut  empty_pc =  (EmptyUnit::new("pc"));
    let mut  empty_reg =  (EmptyUnit::new("register-file"));
    let mut  empty_se =  (EmptyUnit::new("sign-extender"));
    let mut  empty_conc =  (EmptyUnit::new("concater"));
    let mut  empty_ander =  (EmptyUnit::new("ander"));

    //Create mutexes of Arc<empty units>
    let  arc_empty_control: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_control));
    let  arc_empty_alu: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_alu));
    let  arc_empty_add: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_add));
    let  arc_empty_alu_ctrl: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_alu_ctrl));
    let  arc_empty_dm: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_dm));
    let  arc_empty_im: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_im));
    
    let  arc_empty_mux_branch = Arc::new(Mutex::new(empty_mux_branch));
    let  arc_empty_mux_regdst: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_mux_regdst));
    let  arc_empty_mux_jump: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_mux_jump));
    let  arc_empty_mux_alusrc: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_mux_alusrc) );
    let  arc_empty_mux_memtoreg: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_mux_memtoreg));
    let  arc_empty_mux_jr: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_mux_jr));

    let arc_empty_pc: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_pc));
    let arc_empty_reg: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_reg ));
    let arc_empty_se=  Arc::new(Mutex::new(empty_se));
    let arc_empty_conc: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_conc));
    let arc_empty_ander: Arc<Mutex<EmptyUnit>> =  Arc::new(Mutex::new(empty_ander));

    // Create all objects
    let mut pc  = ProgramCounter::new();
    let mut instr_mem = InstructionMemory::new(instructions);
    let mut sign_extend = SignExtend::new();
    let mut alu_add = AddUnit::new();
    let mut alu_control = AluControl::new();
    let mut alu = ALU::new();
    let mut ander = Ander::new();
    let mut data_memory = DataMemory::new();
    let mut registers = Registers::new();
    

    // Create mutexes for "real" objects
    let arc_pc: Arc<Mutex<ProgramCounter>> = Arc::new(Mutex::new(pc));
    let arc_instr_mem: Arc<Mutex<InstructionMemory>> = Arc::new(Mutex::new(instr_mem));
    let arc_sign_extend: Arc<Mutex<SignExtend>> = Arc::new(Mutex::new(sign_extend));
    let arc_alu_add: Arc<Mutex<AddUnit>> = Arc::new(Mutex::new(alu_add));
    let arc_alu_control: Arc<Mutex<AluControl>> = Arc::new(Mutex::new(alu_control));
    let arc_alu: Arc<Mutex<ALU>> = Arc::new(Mutex::new(alu));
    let arc_ander: Arc<Mutex<Ander>> = Arc::new(Mutex::new(ander));
    let arc_data_memory: Arc<Mutex<DataMemory>> = Arc::new(Mutex::new(data_memory));
    let arc_registers: Arc<Mutex<Registers>> = Arc::new(Mutex::new(registers));
    
    //Create real muxes and arcs
    let mux_jr= Mux::new(arc_pc.clone(), PC_IN_ID);
    let arc_mux_jr: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_jr));

    let mux_jump= Mux::new(arc_mux_jr.clone(), MUX_IN_0_ID);
    let arc_mux_jump: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_jump));

    let mux_regdst= Mux::new(arc_registers.clone(), REG_WRITE_REG_ID);
    let arc_mux_regdst: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_regdst));

    let mux_alusrc= Mux::new(arc_alu.clone(), ALU_IN_2_ID);
    let arc_mux_alusrc: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_alusrc));

    let mux_memtoreg= Mux::new(arc_registers.clone(), REG_WRITE_DATA_ID);
    let arc_mux_memtoreg: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_memtoreg));

    let mux_branch = Mux::new(arc_mux_jump.clone(), MUX_IN_0_ID);
    let arc_mux_branch: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_branch));

    
    // Assemble Controller
    let mut control: Control = Control::new(arc_empty_mux_regdst.clone(),
            arc_empty_mux_jump.clone(), 
            arc_empty_mux_jr.clone(), 
            arc_empty_ander.clone(), 
            arc_empty_mux_alusrc.clone(),
            arc_empty_mux_memtoreg.clone(),
            arc_empty_alu_ctrl.clone(),
            arc_empty_reg.clone(),
            arc_empty_dm.clone());

    // Add components to connect with program counter
    arc_pc.lock().unwrap().set_instr_memory(arc_instr_mem.clone()); 
    arc_pc.lock().unwrap().set_concater(arc_empty_conc.clone());
    arc_pc.lock().unwrap().set_add(arc_alu_add.clone());
    arc_pc.lock().unwrap().set_mux_branch(arc_empty_mux_branch.clone());
    /*
    arc_pc.lock().unwrap().set_instr_memory(arc_instr_mem.clone()); 
    arc_pc.lock().unwrap().set_concater(arc_concater.clone());
    arc_pc.lock().unwrap().set_add(arc_alu_add.clone());
    arc_pc.lock().unwrap().set_mux_branch(arc_empty_mux_branch.clone());*/
    // Add components to connect with instruction memory
    arc_instr_mem.lock().unwrap().set_aluctrl(arc_empty_mux_alusrc.clone());
    arc_instr_mem.lock().unwrap().set_concater(arc_empty_conc.clone());
    arc_instr_mem.lock().unwrap().set_control(arc_empty_control.clone());
    arc_instr_mem.lock().unwrap().set_reg(arc_empty_reg.clone());
    arc_instr_mem.lock().unwrap().set_signextend(arc_sign_extend.clone());

    // Add components to connect with sign_extend
    arc_sign_extend.lock().unwrap().set_add(arc_alu_add.clone());

    // Add components to connect with ALU ADD
    arc_alu_add.lock().unwrap().set_mux_branch(arc_empty_mux_branch.clone());

    arc_pc.lock().unwrap().execute();
    arc_instr_mem.lock().unwrap().execute();
    
    arc_sign_extend.lock().unwrap().execute();
    arc_alu_add.lock().unwrap().execute();
    

   /* let instr_mem_clone = arc_instr_mem.clone();

    // Thread for the program counter
    let instruction_thread = thread::spawn(move||{
        let mut instr = instr_mem_clone.lock().unwrap();
        loop {
            instr.execute();
        }
    });
    
    let prog_c_clone = arc_pc.clone();

    // Thread for the instruction memory
    let pc_thread = thread::spawn(move||{
        let mut prog_c = prog_c_clone.lock().unwrap();    
        prog_c.execute(); 
    }); */
    
   // let instr_mem = Mutex::new(Mutex::new(instr_mem));
   // let instr_mem_ref = Mutex::clone(&instr_mem);
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
        Mutex::new(|cc| {
            Mutex::new(MipsApp::new(
                cc,
                labels,
                registers,
                machine_code,
                assembler_code,
            ))
        }),
    );
}*/
