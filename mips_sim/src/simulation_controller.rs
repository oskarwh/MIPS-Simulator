

use std::{thread::{self, sleep}, sync::{Arc, Mutex}, time::Duration, collections::HashMap};
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
    simulation:Option<Simulation>,
}

impl SimulationController {
 
    pub fn new(
    ) -> SimulationController {
        SimulationController{
            simulation:None,
        }
    }

    pub fn start_simulation(&mut self, file_path:&str)->Option<(Vec<u32>, Vec<(String,bool)>, HashMap<String, u32>)>{

        if let Some((machine_code, assembler_code, labels)) = parse_file(file_path){

            // Add vector with machine-code to a vector of Words
            let mut instructions: Vec<Word> = Vec::new();
            for instruction in &machine_code{
                instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
            }

            self.simulation = Some(Simulation::set_up_simulation(instructions));
            self.simulation.as_mut().unwrap().start_simulation();

            return Some((machine_code, assembler_code, labels));

        }else{
            return None;
        }
    }

    pub fn reset_simulation(&mut self, machine_code:&mut Vec<u32>){
        // Add vector with machine-code to a vector of Words
        let mut instructions: Vec<Word> = Vec::new();
        for instruction in machine_code{
            instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
        }
        Simulation::set_up_simulation(instructions);
        self.simulation.as_mut().unwrap().start_simulation();
    }

    //Will be runned from GUI when it wants to step, will update GUI's registers and dm in background
    pub fn step_instruction(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_lock:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
    ) {
        
        self.simulation.as_mut().unwrap().step_simulation(gui_registers, gui_data_memory, gui_pc, gui_lock,gui_changed_dm_index,gui_changed_reg_index);
    
    }


    pub fn run_program(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_lock:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
    ){
        self.simulation.as_mut().unwrap().run_simulation(gui_registers, gui_data_memory, gui_pc, gui_lock,gui_changed_dm_index,gui_changed_reg_index);
     
    }


    
}