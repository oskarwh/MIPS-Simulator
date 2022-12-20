

use std::{thread::{self, sleep}, sync::{Arc, Mutex}, time::Duration};
use crate::{simulation::*, MipsApp, units::data_memory};

use crate::units::program_counter::*;
use crate::units::instruction_memory::*;
use crate::units::add_unit::*;
use crate::units::unit::*;
use crate::units::control::*;
use crate::units::alu::*;
use bitvec::prelude::*;
use eframe::AppCreator;
use egui::Vec2;
use crate::assembler::parse_file;

pub struct SimulationController {
    simulation:Simulation,
}

impl SimulationController {
 
    pub fn new(simulation:Simulation
    ) -> SimulationController {
        SimulationController{
            simulation,
        }
    }

    pub fn start_simulation(&mut self){
        self.simulation.start_simulation();
    }

    //Will be runned from GUI when it wants to step, will update GUI's registers and dm in background
    pub fn step_instruction(&self, registers:Arc<Mutex<Vec<u32>>>, data_memory:Arc<Mutex<Vec<u32>>>) {
        let thread_handle = thread::spawn(move||{
        
            self.simulation.step_simulation();
            let register = self.simulation.get_changed_register();
            let dm = self.simulation.get_changed_memory();

            // Send data to GUI
            registers.lock().unwrap()[register.1] = register.0;
        

        });
    }


    pub fn run_instruction(&self){
        let thread_handle = thread::spawn(move||{
        
            self.simulation.run_simulation();
            
        });
    }
    


}