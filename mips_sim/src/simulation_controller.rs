

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
    default_speed: f32,
    exit_locations: Vec<u32>
}

impl SimulationController {
 
    pub fn new(
    ) -> SimulationController {
        SimulationController{
            simulation:None,
            default_speed: 1000 as f32,
            exit_locations: Vec::new(),
        }
    }

    pub fn setup_simulation(&mut self, file_path:&str)->Option<(Vec<u32>, Vec<(String,bool)>, HashMap<String, u32>, bool)>{

        if let Some((machine_code, assembler_code, labels, contains_error, exit_locations)) = parse_file(file_path){

            // Save exit_locations
            self.exit_locations = exit_locations;

            // Add vector with machine-code to a vector of Words
            let mut instructions: Vec<Word> = Vec::new();
            for instruction in &machine_code{
                instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
            }

            self.simulation = Some(Simulation::set_up_simulation(instructions));
            self.simulation.as_mut().unwrap().start_simulation(self.default_speed);
            return Some((machine_code, assembler_code, labels, contains_error));
        }else{
            return None;
        }
    }

    pub fn reset_simulation(&mut self, machine_code:&mut Vec<u32>){
        //Reset simulation if a simulation is running
        if self.simulation.is_some(){
            self.simulation.as_mut().unwrap().pause_simulation();
        }
        
        // Add vector with machine-code to a vector of Words
        let mut instructions: Vec<Word> = Vec::new();
        for instruction in machine_code{
            instructions.push(instruction.view_bits::<Lsb0>().to_bitvec());
        }
        self.simulation = Some(Simulation::set_up_simulation(instructions));
        self.simulation.as_mut().unwrap().start_simulation(self.default_speed);
    }

    //Will be runned from GUI when it wants to step, will update GUI's registers and dm in background
    pub fn step_instruction(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_found: Arc<Mutex<bool>>
    ) {
        if !self.simulation.as_mut().unwrap().all_instructions_finished(){
            self.simulation.as_mut().unwrap().step_simulation(gui_registers, gui_data_memory, gui_pc, gui_enable,gui_changed_dm_index,gui_changed_reg_index
                , reg_updated_bool, data_updated_bool, &self.exit_locations, exit_found);  
        }
    }


    pub fn run_program(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_simulation_speed: f32,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_found: Arc<Mutex<bool>>
    ){
        if !self.simulation.as_mut().unwrap().all_instructions_finished(){
            //println!("Run");
            self.simulation.as_mut().unwrap().stop_unit_threads();
            self.simulation.as_mut().unwrap().start_simulation(gui_simulation_speed);
            self.simulation.as_mut().unwrap().run_simulation(gui_registers, gui_data_memory, gui_pc, gui_enable,gui_changed_dm_index,gui_changed_reg_index
                , reg_updated_bool, data_updated_bool, &self.exit_locations, exit_found);
        }
    }

    pub fn pause_simulation(&mut self){
        self.simulation.as_mut().unwrap().pause_simulation();
    }



    
}