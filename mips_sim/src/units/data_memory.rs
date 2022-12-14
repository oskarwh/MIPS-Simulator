use std::collections::hash_map;

use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;



pub struct DataMemory<'a> {
    data: hash_map::HashMap<u32, Word>,

    address : u32,
    write_data : Word,

    has_address : bool,
    has_write_data : bool,

    mux_mem_to_reg : Option<&'a mut dyn  Unit>,

    mem_write_signal : bool,
    mem_read_signal : bool,
}


impl DataMemory<'_>{

    pub fn new() -> DataMemory<'static>{
        //Make DataMemory and insert 0 into all of them
        const n_regs:usize = 32;
        let mut data  = hash_map::HashMap::new();

        //Create DataMemory object
        DataMemory{
            data,

            address : 0,
            write_data : bitvec![u32, Lsb0; 0; 32],

            has_address:false,
            has_write_data:false,
            mem_read_signal:false,
            mem_write_signal:false,
            mux_mem_to_reg:None,
        }
    }


    ///Execute unit with thread
    pub fn execute(&mut self){

        if self.has_address && self.has_write_data && self.mem_write_signal{
            //Received reg1! Find corresponding data and send to ALU
            self.data.insert(self.address, self.write_data.to_bitvec());
            
            self.has_address = false;
            self.has_write_data = false;
        }

        if self.has_address  && self.mem_read_signal{
            let data = self.data.get(&self.address).unwrap();
            self.mux_mem_to_reg.as_mut().unwrap().receive(MUX_IN_1_ID, data.to_bitvec());
        }
    }

    /// Set Functions

    pub fn set_mux_mem_to_reg(&mut self, mux: &mut dyn Unit){
        self.mux_mem_to_reg = Some(unsafe { std::mem::transmute(mux) });
    }


}

impl Unit for DataMemory<'_>{

    fn receive(&mut self, input_id: u32, data : Word){
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

    fn receive_signal(&mut self ,signal_id:u32){
        if signal_id == MEM_READ_SIGNAL{
            self.mem_read_signal = true;
        }else if signal_id == MEM_WRITE_SIGNAL{
            self.mem_write_signal = true;
        }
    }
    
}

