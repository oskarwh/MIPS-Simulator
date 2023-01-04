use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;


use crate::units::ander::Ander;
use crate::units::data_memory::DataMemory;
use crate::units::mux::Mux;
use crate::units::program_counter::*;
use crate::units::instruction_memory::*;
use crate::units::add_unit::*;
use crate::units::sign_extend::SignExtend;
use crate::units::unit::*;
use crate::units::control::*;
use super::units::alu::*;
use crate::units::concater::*;

use crate::units::registers::*;
use crate::units::alu_control::*;

/// A single-cycle MIPS simulation. Has one unit-object for each mayor unit in the single-cycle MIPS processor
/// including PC, register-file, instruction-memory, data-memory, ALU, controller, add-unit and alu-controller. 
/// Also has some minor units: ander (used to AND branch-signal from controller with zero-signal from ALU), concater 
/// (used to concat most significant 4 bits from PC+4 with least significant 28 bits from instruction-memory ), 
/// sign-extend (sign extends from 16->32 bits) aswell as branch-mux, jump-mux, jr-mux, regdest-mux, alu-source-mux and 
/// mem-to-reg-mux. 
/// 
/// Units run on their own individual thread. Muxes run on a shared thread. The simulation itself 
/// (run or step instruction) is run on its own thread. Can step one instruction aswell as run 
/// instructions until paused. 
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 
pub struct  Simulation {
    arc_pc: Arc<Mutex<ProgramCounter>>,
    arc_instr_mem: Arc<Mutex<InstructionMemory>>,
    arc_sign_extend: Arc<Mutex<SignExtend>>,
    arc_alu_add: Arc<Mutex<AddUnit>>,
    arc_alu_control: Arc<Mutex<AluControl>>, 
    arc_alu: Arc<Mutex<ALU>>,
    arc_ander: Arc<Mutex<Ander>>,
    arc_data_memory: Arc<Mutex<DataMemory>> ,
    arc_registers: Arc<Mutex<Registers>> ,
    arc_concater: Arc<Mutex<Concater>> ,
    arc_control: Arc<Mutex<Control>> ,

    arc_mux_jr: Arc<Mutex<Mux>>,
    arc_mux_jump: Arc<Mutex<Mux>>,
    arc_mux_regdst: Arc<Mutex<Mux>>,
    arc_mux_alusrc: Arc<Mutex<Mux>>,
    arc_mux_memtoreg: Arc<Mutex<Mux>>,
    arc_mux_branch: Arc<Mutex<Mux>>,

    threads:Vec<thread::JoinHandle<()>>,
    number_of_instructions: u32,

    stop_unit_threads: Arc<Mutex<bool>>,
    stop_run_simulation: Arc<Mutex<bool>>,
}

impl Simulation {

    /// Sets up and returns a new Simulation
    ///
    /// # Arguments
    /// * `instructions` - vector of machine-code instructions
    /// 
    /// # Returns
    ///
    /// * Simulation
    ///
    pub fn set_up_simulation(instructions: Vec<Word>)->Simulation {
    
        // Save numer of instructions
        let number_of_instructions = instructions.len() as u32;

        // Create all objects
        let arc_pc: Arc<Mutex<ProgramCounter>> = Arc::new(Mutex::new(ProgramCounter::new()));
        let arc_instr_mem: Arc<Mutex<InstructionMemory>> = Arc::new(Mutex::new(InstructionMemory::new(instructions)));
        let arc_sign_extend: Arc<Mutex<SignExtend>> = Arc::new(Mutex::new(SignExtend::new()));
        let arc_alu_add: Arc<Mutex<AddUnit>> = Arc::new(Mutex::new(AddUnit::new()));
        let arc_alu_control: Arc<Mutex<AluControl>> = Arc::new(Mutex::new(AluControl::new()));
        let arc_alu: Arc<Mutex<ALU>> = Arc::new(Mutex::new(ALU::new()));
        let arc_ander: Arc<Mutex<Ander>> = Arc::new(Mutex::new(Ander::new()));
        let arc_data_memory: Arc<Mutex<DataMemory>> = Arc::new(Mutex::new(DataMemory::new()));
        let arc_registers: Arc<Mutex<Registers>> = Arc::new(Mutex::new(Registers::new()));
        let arc_concater: Arc<Mutex<Concater>> = Arc::new(Mutex::new(Concater::new()));
        
        let mux_jr= Mux::new(arc_pc.clone(), PC_IN_ID, "jr".to_string());
        let arc_mux_jr: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_jr));

        let mux_jump= Mux::new(arc_mux_jr.clone(), MUX_IN_0_ID, "jump".to_string());
        let arc_mux_jump: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_jump));
       
        let mux_regdst= Mux::new(arc_registers.clone(), REG_WRITE_REG_ID, "regdst".to_string());
        let arc_mux_regdst: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_regdst));
        
        let mux_alusrc= Mux::new(arc_alu.clone(), ALU_IN_2_ID, "alusrc".to_string());
        let arc_mux_alusrc: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_alusrc));
        
        let mux_memtoreg= Mux::new(arc_registers.clone(), REG_WRITE_DATA_ID, "memtoreg".to_string());
        let arc_mux_memtoreg: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_memtoreg));
        
        let mux_branch = Mux::new(arc_mux_jump.clone(), MUX_IN_0_ID, "branch".to_string());
        let arc_mux_branch: Arc<Mutex<Mux>> = Arc::new(Mutex::new(mux_branch));


        // Setup Controller
        let arc_control = Arc::new(Mutex::new(Control::new(
            arc_mux_regdst.clone(),
            arc_mux_jump.clone(), 
            arc_ander.clone(), 
            arc_mux_alusrc.clone(), 
            arc_mux_memtoreg.clone(),
            arc_mux_jr.clone(),
            arc_alu_control.clone(),
            arc_registers.clone(),
            arc_data_memory.clone())));
        
        // Add components to connect with ALU ADD
        arc_alu_add.lock().unwrap().set_mux_branch(arc_mux_branch.clone());

        //Add components to connect with alu controller
        arc_alu_control.lock().unwrap().set_alu(arc_alu.clone());
        
        //Add components to connect with alu
        arc_alu.lock().unwrap().set_mux_mem_to_reg(arc_mux_memtoreg.clone());
        arc_alu.lock().unwrap().set_data_mem_to_reg(arc_data_memory.clone());
        arc_alu.lock().unwrap().set_ander(arc_ander.clone());

        //Add components to ander
        arc_ander.lock().unwrap().set_mux_branch(arc_mux_branch.clone());

        //Add components to connect with concater
        arc_concater.lock().unwrap().set_mux_jump(arc_mux_jump.clone());
        
        //Add components to connect with data-memory
        arc_data_memory.lock().unwrap().set_mux_mem_to_reg(arc_mux_memtoreg.clone());
        
        // Add components to connect with instruction memory
        arc_instr_mem.lock().unwrap().set_aluctrl(arc_alu_control.clone());
        arc_instr_mem.lock().unwrap().set_alu(arc_alu.clone());
        arc_instr_mem.lock().unwrap().set_concater(arc_concater.clone());
        arc_instr_mem.lock().unwrap().set_control(arc_control.clone());
        arc_instr_mem.lock().unwrap().set_reg(arc_registers.clone());
        arc_instr_mem.lock().unwrap().set_signextend(arc_sign_extend.clone());
        arc_instr_mem.lock().unwrap().set_mux_regdst(arc_mux_regdst.clone());

        // Add components to connect with program counter
        arc_pc.lock().unwrap().set_instr_memory(arc_instr_mem.clone()); 
        arc_pc.lock().unwrap().set_concater(arc_concater.clone());
        arc_pc.lock().unwrap().set_add(arc_alu_add.clone());
        arc_pc.lock().unwrap().set_mux_branch(arc_mux_branch.clone());

        //Add components to connect with Registers
        arc_registers.lock().unwrap().set_mux_alu_src(arc_mux_alusrc.clone());
        arc_registers.lock().unwrap().set_alu(arc_alu.clone());
        arc_registers.lock().unwrap().set_data_memory(arc_data_memory.clone());
        arc_registers.lock().unwrap().set_mux_jr(arc_mux_jr.clone());

        // Add components to connect with sign_extend
        arc_sign_extend.lock().unwrap().set_add(arc_alu_add.clone());
        arc_sign_extend.lock().unwrap().set_mux_alu_src(arc_mux_alusrc.clone());

        Simulation {
            arc_pc,
            arc_instr_mem,
            arc_sign_extend,
            arc_alu_add,
            arc_alu_control,
            arc_alu,
            arc_ander,
            arc_data_memory,
            arc_registers,
            arc_concater,
            arc_control,
            threads: Vec::new(),
            //muxes,
            arc_mux_alusrc,
            arc_mux_branch,
            arc_mux_jr,
            arc_mux_jump,
            arc_mux_memtoreg,
            arc_mux_regdst,
            number_of_instructions,

            stop_unit_threads: Arc::new(Mutex::new(false)),
            stop_run_simulation: Arc::new(Mutex::new(false)),

        }
    }


    /// Starts threads for each individual unit and one shared thread for all muxes. Threads will run until manually
    /// stopped.
    /// 
    /// # Arguments
    /// 
    /// * `gui_simulation_speed` - Speed of the simulation
    /// 
    ///   
    /// 
    pub fn start_simulation(&mut self, gui_simulation_speed: f32) {
        //Start simulation
        *self.stop_unit_threads.lock().unwrap() = false;
        *self.stop_run_simulation.lock().unwrap() = false;

        // Start units
        let instr_mem_thread =Self::run_unit_thread(self.arc_instr_mem.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let sign_extend_thread = Self::run_unit_thread(self.arc_sign_extend.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let alu_add_thread = Self::run_unit_thread(self.arc_alu_add.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let alu_control_thread = Self::run_unit_thread(self.arc_alu_control.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let registers_thread  = Self::run_unit_thread(self.arc_registers.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let ander_thread  = Self::run_unit_thread(self.arc_ander.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let alu_thread = Self::run_unit_thread(self.arc_alu.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let dm_thread = Self::run_unit_thread(self.arc_data_memory.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        let concater_thread = Self::run_unit_thread(self.arc_concater.clone(), self.stop_unit_threads.clone(), gui_simulation_speed);
        
        //Start muxes from one thread
        let mut muxes = Vec::new();
        muxes.push(self.arc_mux_branch.clone());
        muxes.push(self.arc_mux_jr.clone());
        muxes.push(self.arc_mux_memtoreg.clone());
        muxes.push(self.arc_mux_alusrc.clone());
        muxes.push(self.arc_mux_regdst.clone());
        muxes.push(self.arc_mux_jump.clone());
        let mux_thread = Self::run_mux_thread(muxes, self.stop_unit_threads.clone());

        //Save thread-handles
        self.threads.push(instr_mem_thread);
        self.threads.push(sign_extend_thread);
        self.threads.push(alu_add_thread);
        self.threads.push(alu_control_thread);
        self.threads.push(registers_thread);
        self.threads.push(ander_thread);
        self.threads.push(alu_thread);
        self.threads.push(dm_thread);
        self.threads.push(concater_thread);
        self.threads.push(mux_thread);

        
    }
    


    /// Step one instruction in the simulation by sending the current address of PC to instruction-memory
    /// and thereby starting one cycle of the processor. The stepping is run on its own thread to make the 
    /// simulation asynchrone of the GUI.
    /// 
    ///  # Arguments
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
    pub fn step_simulation(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_locations: &Vec<u32>,
        exit_found: Arc<Mutex<bool>>
    ) {
        
        if self.arc_pc.lock().unwrap().get_program_count()/4 < self.number_of_instructions {
            Self::step_simulation_thread(gui_registers, gui_data_memory, gui_pc, gui_enable,gui_changed_dm_index, gui_changed_reg_index, 
                self.arc_pc.clone(), self.arc_registers.clone(), self.arc_data_memory.clone(), reg_updated_bool, data_updated_bool,
                exit_locations, exit_found);
        }else {
            *gui_enable.lock().unwrap() = true;
        }
    }
    

    /// Step one instruction in the simulation by sending the current address of PC to instruction-memory
    /// and thereby starting one cycle of the processor. This is the thread function that is run from 
    /// step_simulation. When PC-unit have been executed, waits for instruction to be finished and then updates GUI.
    /// 
    ///  # Arguments
    /// 
    /// * `gui_registers` - Registers from GUI that should be updated
    /// * `gui_data_memory` - Data-memory from GUI that should be updated
    /// * `gui_pc` - Program-count from GUI that should be updated
    /// * `gui_enable` - GUI-enable bool that is set true when the gui should be unlocked
    /// * `gui_changed_dm_index` - The last changed data-memory index that is updated to GUI
    /// * `gui_changed_reg_index` -The last changed register index that is updated to GUI
    /// * `pc` - Reference to PC-unit
    /// * `reg_file` - Reference to register-unit
    /// * `data_memory` - Reference to data-memory unit
    /// * `reg_updated_bool` - Bool that is set to GUI if the register was updated after this function was run
    /// * `data_updated_bool` - Bool that is set to GUI if data-memory was updated after this function was run
    /// 
    ///
    fn step_simulation_thread(
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        pc: Arc<Mutex<ProgramCounter>>,
        reg_file: Arc<Mutex<Registers>>, 
        data_memory: Arc<Mutex<DataMemory>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_locations: &Vec<u32>,
        exit_found: Arc<Mutex<bool>>
        ){
        let mut reg_chain_completed  = false;
        let mut pc_chain_completed = false;

            let simulation_handle = thread::spawn(move||{
            {
            pc.lock().unwrap().execute(); 
            } 
            
            loop {
                // Check when instruction is done (both the PC has received ne address and register-file has
                // received write-data)
                if reg_file.lock().unwrap().instruction_completed(){
                    reg_chain_completed = true;
                }
                if pc.lock().unwrap().has_address(){
                    pc_chain_completed = true;
                }
                if pc_chain_completed && reg_chain_completed {
                    Self::update_gui(
                        gui_registers, 
                        gui_data_memory,
                        gui_pc,
                        gui_changed_dm_index,
                        gui_changed_reg_index,
                        pc,
                        reg_file, 
                        data_memory,
                        reg_updated_bool,
                        data_updated_bool);
                    
                    *gui_enable.lock().unwrap().deref_mut() = true;
                    break;
                }
            }  
        });
    }

    /// Updates the GUI.
    /// 
    ///  # Arguments
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
    fn update_gui( 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        pc: Arc<Mutex<ProgramCounter>>,
        reg_file: Arc<Mutex<Registers>>, 
        data_memory: Arc<Mutex<DataMemory>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
    ){
        // Update data for GUI
        // Update changed register
        let changed_data = reg_file.lock().unwrap().get_changed_register();
        *gui_changed_reg_index.lock().unwrap() = changed_data.1;//update gui with changed index
        gui_registers.lock().unwrap()[changed_data.1] = changed_data.0;

        // Update bool for reg if they have been updated 
        if changed_data.2 { 
            *reg_updated_bool.lock().unwrap() = true;
        }

        // Update changed data memory
        let changed_data = data_memory.lock().unwrap().get_changed_memory();
        *gui_changed_dm_index.lock().unwrap() = changed_data.1;//update gui with changed index
        gui_data_memory.lock().unwrap()[changed_data.1] = changed_data.0;
        
        // Update bool for data if they have been updated 
        if changed_data.2 {
            *data_updated_bool.lock().unwrap() = true;
        }

        // Update PC 
        *gui_pc.lock().unwrap() = pc.lock().unwrap().get_program_count()/4;
        
    }


    /// Execute instructions until finished or paused by sending the current address of PC to instruction-memory
    /// and thereby starting one cycle of the processor. Runs on its own thread to make the 
    /// simulation asynchrone of the GUI.
    /// 
    ///  # Arguments
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
    pub fn run_simulation(&mut self, 
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_locations: &Vec<u32>,
        exit_found: Arc<Mutex<bool>>
    ) {
        if self.arc_pc.lock().unwrap().get_program_count()/4 < self.number_of_instructions {
            *self.stop_run_simulation.lock().unwrap() = false;
            Self::run_simulation_thread(gui_registers, gui_data_memory, gui_pc, gui_enable,gui_changed_dm_index,gui_changed_reg_index, 
                self.arc_pc.clone(), self.arc_registers.clone(), self.arc_data_memory.clone(), self.number_of_instructions, 
    self.stop_run_simulation.clone(), reg_updated_bool, data_updated_bool, exit_locations, exit_found);
        }else {
            *gui_enable.lock().unwrap() = true;
        }
    }

    /// Execute instructions until finished or paused by sending the current address of PC to instruction-memory
    /// and thereby starting one cycle of the processor. This is the thread-function that is run from 
    /// run_simulation. When PC-unit have been executed, waits for instruction to be finished and then updates GUI.
    /// 
    ///  # Arguments
    /// 
    /// * `gui_registers` - Registers from GUI that should be updated
    /// * `gui_data_memory` - Data-memory from GUI that should be updated
    /// * `gui_pc` - Program-count from GUI that should be updated
    /// * `gui_enable` - GUI-enable bool that is set true when the gui should be unlocked
    /// * `gui_changed_dm_index` - The last changed data-memory index that is updated to GUI
    /// * `gui_changed_reg_index` -The last changed register index that is updated to GUI
    /// * `pc` - Reference to PC-unit
    /// * `reg_file` - Reference to register-unit
    /// * `data_memory` - Reference to data-memory
    /// * `n_instructions` - Total number of instructions currently in the simulation
    /// * `stop_run` - Bool to stop the running of the simulation (the execution of instructions and not unit threads)
    /// * `reg_updated_bool` - Bool that is set to GUI if the register was updated after this function was run
    /// * `data_updated_bool` - Bool that is set to GUI if data-memory was updated after this function was run
    /// 
    ///
    fn run_simulation_thread(
        gui_registers:Arc<Mutex<Vec<i32>>>, 
        gui_data_memory:Arc<Mutex<Vec<i32>>>,
        gui_pc:Arc<Mutex<u32>>,
        gui_enable:Arc<Mutex<bool>>,
        gui_changed_dm_index:Arc<Mutex<usize>>,
        gui_changed_reg_index:Arc<Mutex<usize>>,
        pc: Arc<Mutex<ProgramCounter>>,
        reg_file: Arc<Mutex<Registers>>, 
        data_memory: Arc<Mutex<DataMemory>>,
        n_instructions: u32,
        stop_run:Arc<Mutex<bool>>,
        reg_updated_bool:Arc<Mutex<bool>>,
        data_updated_bool:Arc<Mutex<bool>>,
        exit_locations: &Vec<u32>,
        exit_found: Arc<Mutex<bool>>
    ){
        
        let mut reg_chain_completed  = false;
        let mut pc_chain_completed = false;
            let simulation_handle = thread::spawn(move||{
            loop {
                {
                    pc.lock().unwrap().execute(); 
                }  

                loop {
                    // Check when instruction is done
                    if reg_file.lock().unwrap().instruction_completed(){
                        reg_chain_completed = true;
                    }
                    if pc.lock().unwrap().has_address(){
                        pc_chain_completed = true;
                    }
                    if pc_chain_completed && reg_chain_completed   {
                        Self::update_gui(
                            gui_registers.clone(), 
                            gui_data_memory.clone(),
                            gui_pc.clone(),
                            gui_changed_dm_index.clone(),
                            gui_changed_reg_index.clone(),
                            pc.clone(),
                            reg_file.clone(), 
                            data_memory.clone(),
                            reg_updated_bool.clone(),
                            data_updated_bool.clone());
                            
                        pc_chain_completed = false;
                        reg_chain_completed = false;
                        break;
                    }
                }
                // Check if all instructions is done or simulation is paused
                if Self::get_program_count_index(pc.clone()) >= n_instructions {
                    *gui_enable.lock().unwrap().deref_mut() = true;
                    break;
                }else if *stop_run.lock().unwrap(){
                    *gui_enable.lock().unwrap().deref_mut() = true;
                    break;
                }
            }
        });
    }

    
    /// Runs the execute function for a unit on a separate thread
    /// 
    /// # Arguments
    /// 
    /// * `thread` - The unit that is run
    /// * `stop` - Bool to stop the thread
    /// * `simulation_speed` - Speed of the thread
    /// 
    /// 
    /// # Returns
    ///
    /// * JoinHandle - Handle to the unit-thread
    ///   
    fn run_unit_thread(thread: Arc<Mutex<dyn Unit>>, stop: Arc<Mutex<bool>>, simulation_speed: f32)->thread::JoinHandle<()>{
    
        let thread_handle = thread::spawn(move||{
        
            //Run unit until stopped
            while !*stop.lock().unwrap() {
                {
                    let mut temp = thread.lock().unwrap();
                    temp.execute();
                }

                //Calculates the sleep-time with a functions that converts instructions/second to sleep-time
                let sleep_time = 371.25760622*simulation_speed.powf(-1.42121824);
                sleep(Duration::from_millis(sleep_time as u64));
            }
        });
        thread_handle
    } 

    /// Runs the execute function for all muxes on a separate thread
    /// 
    /// # Arguments
    /// 
    /// * `muxes` - Vector of muxes
    /// * `stop` - Bool to stop the thread
    /// 
    /// 
    /// # Returns
    ///
    /// * JoinHandle - Handle to the thread
    ///   
    fn run_mux_thread( muxes:Vec<Arc<Mutex<Mux>>>, stop: Arc<Mutex<bool>>)->thread::JoinHandle<()>{
        let thread_handle = thread::spawn(move||{

            //Run unit until stopped
            while !*stop.lock().unwrap() {
                for mux in &muxes{
                    let mut temp = mux.lock().unwrap();
                    temp.execute();
                }
            }
        });
        thread_handle
    } 

    /// Stops the simulation thread
    ///  
    pub fn pause_simulation(&mut self){
       *self.stop_run_simulation.lock().unwrap() = true;
       
    }
    
    /// Stops all unit and mux threads
    ///  
    pub fn stop_unit_threads(&mut self){
        *self.stop_unit_threads.lock().unwrap() = true;
        while let Some(thread) = self.threads.pop(){
            thread.join().unwrap();
        }
    }

    /// Returns current PC-address divided by 4 (word address)
    /// 
    /// # Arguments
    /// 
    /// * `pc` - PC-unit
    /// 
    /// # Returns
    ///
    /// * u32 - the current word-address of the PC
    ///  
    fn get_program_count_index(pc: Arc<Mutex<ProgramCounter>>)->u32{
        pc.lock().unwrap().get_program_count()/4
    }

    /// Returns true if the instructions of the current simulation is finished (have reached last address in instruction
    /// memory)
    /// 
    /// # Returns
    ///
    /// * bool - true if PC have reached last address in instruction memory, otherwise false
    ///  
    pub fn all_instructions_finished(&self)->bool{
        Self::get_program_count_index(self.arc_pc.clone()) > self.number_of_instructions
    }

}



