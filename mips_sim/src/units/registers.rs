use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Mutex;
use std::sync::Arc;

/// A MIPS simulator unit. A Register File which holds 32 readable and writable registers, all
/// holding an word each.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-01-03: First complete version.
/// 

/// Register Struct
pub struct Registers {

    instruction_complete: bool,
    prev_register_index: usize,

    registers: Vec<Word>, 

    read1_reg : u32,
    read2_reg : u32,
    write_reg : u32,
    write_data : Word,

    has_read1 : bool,
    has_read2 : bool,
    has_write_reg : bool,
    has_write_data : bool,
    reg_updated: bool,


    alu : Option<Arc<Mutex<dyn Unit>>>,
    mux_alu_src : Option<Arc<Mutex<dyn Unit>>>,
    mux_jr : Option<Arc<Mutex<dyn Unit>>>,
    data_memory : Option<Arc<Mutex<dyn Unit>>>,

    reg_write_signal : bool,
}

/// Registers Implementation
impl Registers {

    /// Returns a new Registers.
    /// 
    /// # Returns
    ///
    /// * Registers
    ///
    pub fn new() -> Registers{
        //Make registers and insert 0 into all of them
        const N_REGS:usize = 32;
        let mut registers: Vec<Word> = vec![bitvec![u32, Lsb0; 0; 32]; N_REGS];

        //Create registers object
        Registers{
            instruction_complete: false,
            prev_register_index: 0,

            registers,
            
            has_read1:false,
            has_read2:false,
            has_write_reg:false,
            has_write_data:false,
            reg_write_signal:false,
            reg_updated:false,

            read1_reg: 0,
            read2_reg: 0,
            write_reg: 0,
            write_data: bitvec![u32, Lsb0; 0; 32],

            alu: None,
            mux_alu_src: None,
            data_memory: None,
            mux_jr: None,
        }
    }


    /// Set a Alu that the 'Registers' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `alu` - The Alu that should be set
    ///
    pub fn set_alu(& mut self, alu: Arc<Mutex<dyn Unit>>){
        self.alu = Some(alu);
    }

    /// Set which Mux that the 'Registers' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_alu_src(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_alu_src = Some(mux);
    }

    /// Set a DataMemory that the 'Registers' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `data_memory` - The DataMemory that should be set
    ///
    pub fn set_data_memory(&mut self, data_memory: Arc<Mutex<dyn Unit>>){
        self.data_memory = Some(data_memory);
    }

    /// Set which Mux that the 'Registers' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_jr(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_jr = Some(mux);
    }

    /// Returns the recently changed register and the corresponding index.
    /// 
    /// # Returns
    /// 
    /// * (i32, usize, bool) - Register value, Register Index, Boolean indicating if 
    /// change occured on current instruction.
    pub fn get_changed_register(&mut self) -> (i32, usize, bool) {
        let temp = (self.registers[self.prev_register_index].clone().into_vec()[0] as i32, self.prev_register_index, self.reg_updated);

         // Reset bool when GUI has gotten data.
         self.reg_updated = false;

         return temp;
    }

    /// Checks if instruction has been completed by checking if Register File has received a data to write.
    /// 
    /// # Returns
    /// 
    /// * bool - Boolean that holds true if instruction is complete.
    pub fn instruction_completed(&mut self) -> bool {
        if self.instruction_complete {
            self.instruction_complete = false;          
            true
        }else {
            false
        }    
    }


}

/// Registers implementing Unit trait.
impl Unit for Registers {

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id ==  REG_READ_1_ID{
            self.read1_reg = data.to_bitvec().into_vec()[0];
            self.has_read1 = true;
        }else if input_id ==  REG_READ_2_ID{
            self.read2_reg = data.to_bitvec().into_vec()[0];
            self.has_read2 = true;
        }else if input_id ==  REG_WRITE_REG_ID{
            self.write_reg = data.to_bitvec().into_vec()[0];
            self.has_write_reg = true;
        }else if input_id ==  REG_WRITE_DATA_ID{
            self.write_data = data;
            self.has_write_data = true;
            // Instruction about to be completed
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
    fn receive_signal(&mut self ,signal_id:u32, signal: bool){
        if signal_id == DEFAULT_SIGNAL{
            self.reg_write_signal = signal;
            
        }
    }

    /// Checks if data input has been received, corresponding to index that should be forwarded/overwritten.
    /// If that is the case Registers will forward register value to correct Unit or overwrite correct register with incoming data.
    fn execute(&mut self){

        if self.has_read1{
            //Received reg1! Find corresponding data and send to ALU
            let mut data = self.registers[self.read1_reg as usize].to_bitvec();

            self.alu.as_mut().unwrap().lock().unwrap().receive(ALU_IN_1_ID, data.to_bitvec());

            // Send multiplied by 4 to Jump Register mux
            data.shift_right(2);
            self.mux_jr.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data.to_bitvec());
            self.has_read1 = false;
        }

        if self.has_read2{
            //Received reg1! Find corresponding data and send to ALU-src mux and Data Memory
            let data = self.registers[self.read2_reg as usize].to_bitvec();
            self.mux_alu_src.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID, data.to_bitvec());
            self.data_memory.as_mut().unwrap().lock().unwrap().receive(DM_DATA_ID, data.to_bitvec());
            self.has_read2 = false;
        }

        if self.has_write_reg && self.has_write_data {
            //Write if signal is on and the register isnt the $zero reg
            if self.reg_write_signal && self.write_reg != 0{
                // Reset register bool
                self.prev_register_index = self.write_reg as usize;
    
                //Got data to write and is signaled to write! Insert into registers on index write_reg
                self.registers[self.write_reg as usize] = self.write_data.to_bitvec();
                
                // Set bool to true to let GUI know register has been updated
                self.reg_updated = true;
            }
            //register have received write data (indicates that instruction is done)
            self.has_write_reg = false;
            self.has_write_data = false;
            self.instruction_complete = true;  
        }
    }
}




