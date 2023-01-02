#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod simulation;
mod units;
pub use app::MipsApp;
pub mod assembler;
pub mod simulation_controller;

use crate::simulation::*;
use crate::simulation_controller::*;
use mips_sim::*;
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
use units::unit;

use crate::units::add_unit::*;
use crate::units::alu::*;
use crate::units::control::*;
use crate::units::instruction_memory::*;
use crate::units::program_counter::*;
use crate::units::unit::*;
use assembler::parse_file;
use bitvec::prelude::*;
use egui::Vec2;

use bitvec::view::BitView;
use eframe::AppCreator;

use crate::units::{
    alu_control::AluControl,
    ander::Ander,
    data_memory::DataMemory,
    mux::Mux,
    registers::Registers,
    sign_extend::{self, SignExtend},
};

const DEFAULT_WIDTH: f32 = 1000.0;
const DEFAULT_HEIGHT: f32 = 600.0;
const MIN_WIDTH: f32 = 800.0;
const MIN_HEIGHT: f32 = 600.0;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(None),
        min_window_size: Option::from(Vec2::new(MIN_WIDTH, MIN_HEIGHT)),
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
        mouse_passthrough: false,
        event_loop_builder: None,
        shader_version: None,
        centered: true,
    };

    let sim_controller = SimulationController::new();

    eframe::run_native(
        "MIPS Simulator",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc, sim_controller))),
    );
}
