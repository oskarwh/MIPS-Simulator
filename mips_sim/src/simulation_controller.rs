

use std::{thread::{self, sleep}, sync::{Arc, Mutex}, collections::HashMap};
use crate::{simulation::*};


use crate::units::unit::*;
use bitvec::prelude::*;
use crate::assembler::parse_file;



/// Controller for the MIPS simulation. Can start a new simulation, reset the simulation, run program program specified
/// by current machine code file and step one instruction in the current program
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 
pub struct SimulationController {
    simulation:Option<Simulation>,
    default_speed: f32,
    exit_locations: Vec<u32>
}

impl SimulationController {

  
    /// Returns a new Simulation Controller
    ///
    /// # Arguments
    /// 
    /// # Returns
    ///
    /// * SimulationController
    ///
    pub fn new(
    ) -> SimulationController {
        SimulationController{
            simulation:None,
            default_speed: 1000 as f32,
            exit_locations: Vec::new(),
        }
    }

    /// Receives a file-path to an assembly code file. Sets up a new MIPS-simulation that has assembled machine code
    /// in it's instruction-memory. Returns machine-code, assembler-code, labels, and a bool which tells if the assembler
    /// code contains errors wrapped in an Option. 
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - file-path to an assembler-code file
    /// 
    /// # Returns
    ///
    /// * Option<(Vec<u32>, Vec<(String,bool)>, HashMap<String, u32>, bool)> - machine-code, assembler-code, labels,
    ///  and a bool which tells if the assembler code contains errors wrapped in an Option.
    ///   
    /// 
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

    /// Resets and restarts the simulation with a new machine code inserted into instruction-memory
    /// 
    /// # Arguments
    /// 
    /// * `machine_code` - vector with machine code
    /// 
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

    /// Step one instruction in the current simulation. Runs asyncronically from another thread inside simulation
    /// and thereby needs references to attributes from the caller (the GUI) that should be updated.
    /// 
    /// # Arguments
    /// 
    /// * `gui_registers` - Registers from GUI that should be updated
    /// * `gui_data_memory` - Data-memory from GUI that should be updated
    /// * `gui_pc` - Program-count from GUI that should be updated
    /// * `gui_enable` - GUI-enable bool that is set true when the gui should be unlocked
    /// * `gui_changed_dm_index` - The last changed data-memory index that is updated to GUI
    /// * `gui_changed_reg_index` -The last changed register index that is updated to GUI
    /// * `reg_updated_bool` - Bool that is set to GUI if the register was updated after this function was run
    /// * `data_updated_bool` - Bool that is set to GUI if data-memory was updated after this function was run
    /// 
    ///
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

    /// Run the current simulation with a certain speed. Runs asyncronically from another thread inside simulation
    /// and thereby needs references to attributes from the caller (the GUI) that should be updated.
    /// 
    /// # Arguments
    /// 
    /// * `gui_registers` - Registers from GUI that should be updated
    /// * `gui_data_memory` - Data-memory from GUI that should be updated
    /// * `gui_pc` - Program-count from GUI that should be updated
    /// * `gui_enable` - GUI-enable bool that is set true when the gui should be unlocked
    /// * `gui_simulation_speed` - Run-speed for the simulation
    /// * `gui_changed_dm_index` - The last changed data-memory index that is updated to GUI
    /// * `gui_changed_reg_index` -The last changed register index that is updated to GUI
    /// * `reg_updated_bool` - Bool that is set to GUI if the register was updated after this function was run
    /// * `data_updated_bool` - Bool that is set to GUI if data-memory was updated after this function was run
    /// 
    ///
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

    /// 
    /// Pauses the current running of a simulation
    /// 
    ///
    pub fn pause_simulation(&mut self){
        self.simulation.as_mut().unwrap().pause_simulation();
    }



    
}