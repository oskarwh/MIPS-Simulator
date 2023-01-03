#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod simulation;
mod units;
pub use app::MipsApp;
pub mod assembler;
pub mod simulation_controller;
use crate::simulation_controller::*;
use egui::Vec2;

const MIN_WIDTH: f32 = 800.0;
const MIN_HEIGHT: f32 = 600.0;

/// Main program for a single-cycle MIPS simulation. GUI is run on separate thread and is in charge of the 
/// whole program. GUI uses a simulation-controller to control the MIPS-simulation.
/// 
/// Main starts the GUI.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {

    //options for the GUI
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

    eframe::run_native(
        "MIPS Simulator",
        native_options,
        Box::new(|cc| Box::new(MipsApp::new(cc, SimulationController::new()))),
    );
}
