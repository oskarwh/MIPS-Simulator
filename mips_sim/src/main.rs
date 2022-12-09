/*#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use mips_sim::*;
use std::{thread::{self, sleep}, sync::{Arc, Mutex}};

#[path = "units/unit.rs"]mod unit;
#[path = "units/program_counter.rs"]mod program_counter;
#[path = "units/instruction_memory.rs"]mod instruction_memory;

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
    pc.set_add(empty_unit{});
    pc.set_concater(empty_unit{});
    pc.set_mux_branch(empty_unit{});
    pc.set_instr_memory(&instr_mem);

    // Add components to connect with instruction memory
    instr_mem.set_pc(&pc);
    instr_mem.set_control(empty_unit{});
    instr_mem.set_add(empty_unit{});
    instr_mem.set_reg(empty_unit{});
    instr_mem.set_signextend(empty_unit{});
    instr_mem.set_aluctrl(empty_unit{});
    instr_mem.set_concater(empty_unit{});


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
        initial_window_size: Option::from(Vec2::new(1000 as f32, 800 as f32)),
        min_window_size: Option::from(Vec2::new(600 as f32, 400 as f32)),
        max_window_size: Option::from(Vec2::new(1000 as f32, 800 as f32)),
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
