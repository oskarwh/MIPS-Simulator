use std::collections::hash_map;
use std::sync::Arc;
use std::sync::Mutex;

use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;

const MAX_WORDS: usize = 250;

pub struct DataMemory {
    data: Vec<Word>,

    address : u32,
    write_data : Word,

    has_address : bool,
    has_write_data : bool,
    has_mem_read_signal:bool,
    has_mem_write_signal:bool,

    mux_mem_to_reg : Option<Arc<Mutex<dyn Unit>>>,

    mem_write_signal : bool,
    mem_read_signal : bool,

    //Last changed memory
    prev_memory_index: usize,
    prev_data: i32,
}


impl DataMemory{

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

            mem_read_signal:false,
            mem_write_signal:false,
            mux_mem_to_reg:None,

            prev_memory_index: 0,
            prev_data: 0,

        }
    }

    //Get last changed register
    pub fn get_changed_memory(&self) -> (i32,usize) {
        (self.prev_data, self.prev_memory_index)
    }


    /// Set Functions

    pub fn set_mux_mem_to_reg(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_mem_to_reg = Some(mux);
    }

    fn reset_bools(&mut self){
        self.has_address = false;
        self.has_write_data = false;
        self.has_mem_read_signal = false;
        self.has_mem_write_signal = false;
    }



}

impl Unit for DataMemory{

    fn receive(&mut self, input_id: u32, data : Word){
        println!("\t DM received {} from {}", data, input_id);
        if input_id ==  DM_ADDR_ID{
            self.address = data.into_vec()[0];
            self.has_address = true;
        }else if input_id ==  REG_READ_2_ID{
            self.write_data = data;
            self.has_write_data = true;
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool){
        println!("\t DM received signal {} as {}", signal_id, signal);
        if signal_id == MEM_READ_SIGNAL{
            self.mem_read_signal = signal;
            self.has_mem_read_signal = true;
        }else if signal_id == MEM_WRITE_SIGNAL{
            self.mem_write_signal = signal;
            self.has_mem_write_signal = true;
        }
    }

    ///Execute unit with thread
    fn execute(&mut self){

        if self.has_address && self.has_write_data && self.has_mem_read_signal && self.has_mem_write_signal{
            if self.mem_write_signal{
                
                self.data[self.address as usize] = self.write_data.to_bitvec();
                self.prev_memory_index = self.address as usize;
                self.prev_data = self.write_data.to_bitvec().into_vec()[0] as i32;
              
            }else if self.mem_read_signal{
                let data = self.data[self.address as usize].to_bitvec();
                self.mux_mem_to_reg.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data);
                
            }else{
                //Send shit value to mux to make it stop waiting
                let data = bitvec![u32, Lsb0; 0; 32];
                self.mux_mem_to_reg.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data);
            }
            self.reset_bools();
        }
    }
    
}

