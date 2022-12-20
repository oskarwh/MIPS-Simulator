#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod simulation;
mod units;
pub use app::MipsApp;
pub mod assembler;
pub mod simulation_controller;
