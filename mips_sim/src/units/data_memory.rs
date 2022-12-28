use std::sync::Arc;
use std::sync::Mutex;
use bitvec::prelude::*;
use crate::units::unit::*;

/// A MIPS simulator unit. Holds a memory of predefined amount of words, data memory
/// is able to load and store words from memory.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// Max amount of words in memory.
const MAX_WORDS: usize = 250;

/// DataMemory Struct
pub struct DataMemory {
    data: Vec<Word>,

    address : u32,
    write_data : Word,

    has_address : bool,
    has_write_data : bool,
    has_mem_read_signal:bool,
    has_mem_write_signal:bool,
    data_updated: bool,

    mux_mem_to_reg : Option<Arc<Mutex<dyn Unit>>>,

    mem_write_signal : bool,
    mem_read_signal : bool,

    prev_memory_index: usize,
    prev_data: i32,
}

/// DataMemory Implementation
impl DataMemory{

    /// Returns a new DataMemory.
    ///
    /// # Returns
    ///
    /// * DataMemory
    ///
    pub fn new() -> DataMemory{
        //Make DataMemory and insert 0 into all of them
        const n_regs:usize = 32;
        // Create array with zeros
        let mut data = vec![bitvec![u32, Lsb0; 0; 32]; MAX_WORDS];
        //Create DataMemory object
        DataMemory{
            data: data,

            address : 0,
            write_data : bitvec![u32, Lsb0; 0; 32],

            has_address:false,
            has_write_data:false,
            has_mem_read_signal:false,
            has_mem_write_signal:false,
            data_updated: false,

            mem_read_signal:false,
            mem_write_signal:false,
            mux_mem_to_reg:None,

            prev_memory_index: 0,
            prev_data: 0,

        }
    }

    /// Returns last changed index in memory and the corresponding data, also returns a boolean
    /// to know wether or not the change occured during the current instruction or not.
    /// 
    /// # Returns
    /// 
    /// * (i32, usize, bool) - Data as a Word, Index in memory, Boolen if change occured during current instruction
    pub fn get_changed_memory(&mut self) -> (i32, usize, bool) {
        let temp = (self.prev_data, self.prev_memory_index, self.data_updated);

        // Reset bool when GUI has gotten data.
        self.data_updated = false;

        return temp;
    }

    /// Set which Mux that the 'DataMemory' which is called on, should send data from memory to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_mem_to_reg(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_mem_to_reg = Some(mux);
    }

    /// Resets bools that holds wether or not incoming signals and function code has been recived.
    fn reset_bools(&mut self){
        self.has_address = false;
        self.has_write_data = false;
        self.has_mem_read_signal = false;
        self.has_mem_write_signal = false;
    }

}

/// AddUnit implementing Unit trait.
impl Unit for DataMemory{

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        //println!("\t DM received {} from {}", data, input_id);
        if input_id ==  DM_ADDR_ID{
            self.address = data.into_vec()[0]/4;
            self.has_address = true;
        }else if input_id ==  REG_READ_2_ID{
            self.write_data = data;
            self.has_write_data = true;
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
        //println!("\t DM received signal {} as {}", signal_id, signal);
        if signal_id == MEM_READ_SIGNAL{
            self.mem_read_signal = signal;
            self.has_mem_read_signal = true;
        }else if signal_id == MEM_WRITE_SIGNAL{
            self.mem_write_signal = signal;
            self.has_mem_write_signal = true;
        }
    }

    /// Checks if all data and signals needed has been received.
    /// If that is the case check wether to Load a Word from a given index in memory or store a incoming
    /// Word to given a index in memory.
    fn execute(&mut self){

        if self.has_address && self.has_write_data && self.has_mem_read_signal && self.has_mem_write_signal{
            if self.mem_write_signal{
                
                self.data[self.address as usize] = self.write_data.to_bitvec();
                self.prev_memory_index = self.address as usize;
                self.prev_data = self.write_data.to_bitvec().into_vec()[0] as i32;
                //println!("\t DM WRITING DATA, {} on index {}", self.prev_data, self.address);

                // Set bool to let GUI know memory has been updated
                self.data_updated = true;
              
            }else if self.mem_read_signal{
                let data = self.data[self.address as usize].to_bitvec();
                self.mux_mem_to_reg.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data);
                
            }
            if !self.mem_read_signal {
                //Send shit value to mux to make it stop waiting
                let data = bitvec![u32, Lsb0; 0; 32];
                self.mux_mem_to_reg.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data);
            }
            self.reset_bools();
        }
    }
    
}

