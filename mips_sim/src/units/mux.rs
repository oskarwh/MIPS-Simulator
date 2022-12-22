
use std::sync::{Arc, Mutex};

use crate::units::unit::*;
use bitvec::prelude::*;

pub struct Mux {
    data0 : Word,
    data1 : Word,

    signal : bool,
    output_unit :Arc<Mutex<dyn Unit>>,
    output_id : u32,

    has_val0 : bool,
    has_val1 : bool,
    has_signal :bool,
    name: String,
}

impl Mux {


    pub fn new(out: Arc<Mutex<dyn Unit>>, out_id : u32, name:String) -> Mux{
        
        Mux{
            output_unit: out,
            output_id: out_id,
            signal : false,
            data0: bitvec![u32, Lsb0; 0; 32],
            data1: bitvec![u32, Lsb0; 0; 32],
            has_val0: false,
            has_val1: false,
            has_signal: false,
            name
        }
    }

    fn reset_bools(&mut self) {
        self.has_val0 = false;
        self.has_val1 = false;
        self.has_signal= false;
    }


}

impl Unit for Mux{
    fn receive(&mut self, input_id: u32, data : Word ){
        println!("\t mux {} received data {} from {}",self.name, data, input_id);
        if input_id == MUX_IN_0_ID{
            self.data0 = data;
            self.has_val0 = true;
        }else if input_id == MUX_IN_1_ID {
            self.data1 = data;
            self.has_val1 = true;
        }else{
            //Data came on undefined input_id
        }
    }


    fn receive_signal(&mut self ,signal_id:u32, signal: bool){
        println!("\t mux {} received signal {} with id {}",self.name, signal, signal_id);
        self.signal = signal;
        self.has_signal = true;
    }

    fn execute(&mut self){
        // Some type of loop so the signal doesnt go unnoticed
        if self.has_val0 && self.has_val1 && self.has_signal{
            if self.signal{
                self.output_unit.lock().unwrap().receive(self.output_id, self.data1.to_bitvec());
            }else{
                self.output_unit.lock().unwrap().receive(self.output_id, self.data0.to_bitvec());
            }
            self.reset_bools();
            
        }

    }
}












