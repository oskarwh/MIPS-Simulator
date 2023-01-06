use crate::units::unit::*;
use bitvec::prelude::*;
use std::sync::{Mutex, Arc};

/// A MIPS simulator unit. Forwards which instruction in the form of a index that 
/// the InstructionMemory should run.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 

/// ProgramCounter Struct 
pub struct ProgramCounter {
    has_address: bool,
    current_address : Word,
    instruction_memory : Option<Arc<Mutex<dyn Unit>>>,
    concater : Option<Arc<Mutex<dyn Unit>>>,
    add_unit : Option<Arc<Mutex<dyn Unit>>>,
    mux_branch: Option<Arc<Mutex<dyn Unit>>>,
}

/// ProgramCounter Implementation
impl<'a>  ProgramCounter {

    /// Returns a new ProgramCounter.
    /// 
    /// # Returns
    ///
    /// * ProgramCounter
    ///
    pub fn new() -> ProgramCounter {
        ProgramCounter{
            current_address : bitvec![u32, Lsb0; 0; 32],
            has_address : true,

            instruction_memory: None,
            concater: None,
            add_unit: None,
            mux_branch: None,
        }
    }

    /// Adds four to the current address.
    ///
    /// # Arguments
    /// 
    /// * `addr` - Address to use
    /// 
    /// # Returns
    ///
    /// * Word - Result of addition.
    ///
    fn add_4(addr : Word) -> Word {
        let num = addr.into_vec()[0];
        let res = num + 4;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    /// Set a InstructionMemory that the 'ProgramCounter' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `instr_mem` - The InstructionMemory that should be set
    ///
    pub fn set_instr_memory(&'a mut self, instr_mem: Arc<Mutex<dyn Unit>>) {
        self.instruction_memory = Some(instr_mem);
    }

    /// Set a Concater that the 'ProgramCounter' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `concater` - The Concater that should be set
    ///
    pub fn set_concater(&'a mut self, concater: Arc<Mutex<dyn Unit>>) {
        self.concater = Some(concater);
    }

    /// Set a AddUnit that the 'ProgramCounter' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `add` - The AddUnit that should be set
    ///
    pub fn set_add(&'a mut self, add: Arc<Mutex<dyn Unit>>) {
        self.add_unit = Some(add);
    }

    /// Set which Mux that the 'ProgramCounter' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_branch(&'a mut self, mux: Arc<Mutex<dyn Unit>>) {
        self.mux_branch = Some(mux);
    }

    /// Returns current address/index.
    /// 
    /// # Returns
    /// 
    /// * u32 - Current address
    pub fn get_program_count(&self) -> u32 {
        self.current_address.clone().into_vec()[0]
    }

    /// Returns true if ProgramCounter has a address to use.
    /// 
    /// # Returns
    /// 
    /// * bool - True if there is a address otherwise false
    pub fn has_address(&self)->bool{
        self.has_address
    }
}

/// ProgramCounter implementing Unit trait.
impl Unit for ProgramCounter {

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, address : Word){  
        if input_id == PC_IN_ID{
            self.current_address = address;
            self.has_address = true;
        }else{
            //Message came on undefined input
        }  
    }

    /// Receives signal from a Control, comes with ID to 
    /// specify which signal.
    /// 
    /// # Arguments
    /// 
    /// * `signal_id` - Id to know what type of signal is comming
    /// * `signal` - Bool which holds state of signal (high/low)
    /// 
    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        todo!()
    }

    /// Checks if a address is found.
    /// If that is the case send address to InstructionMemory and send data to
    /// Untis to calculate next address.
    fn execute(&mut self){
        if self.has_address {
            //Send address to instruction memory
            self.instruction_memory.as_mut().unwrap().lock().unwrap().receive(IM_READ_ADDRESS_ID, self.current_address.to_bitvec());

            //add 4 to address
            let added_address = Self::add_4(self.current_address.to_bitvec());

            //Send added address to concater
            self.concater.as_mut().unwrap().lock().unwrap().receive(CONC_IN_2_ID, added_address[28..32].to_bitvec());

            //Send added address to add-unit
            self.add_unit.as_mut().unwrap().lock().unwrap().receive(ADD_IN_1_ID,added_address.to_bitvec());

            //Send added address to mux-branch
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID, added_address.to_bitvec());

            self.has_address = false;
        }

    }
}

