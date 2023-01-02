use std::sync::{Arc, Mutex};
use crate::units::unit::*;
use bitvec::prelude::*;

/// A MIPS simulator unit. Recives two Bit Vectors and will choose one 
/// to forward to choosen Unit by looking at incoming signal.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 

/// Mux Struct
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

/// Mux Implementation
impl Mux {

    /// Returns a new Mux.
    ///
    /// # Arguments
    /// 
    /// * `out` - Unit to forward data to
    /// * `out_id` - ID to send use when forwarding to choosen Unit
    /// * `name` - Name of Mux
    /// 
    /// # Returns
    ///
    /// * Mux
    ///
    pub fn new(out: Arc<Mutex<dyn Unit>>, out_id : u32, name:String) -> Mux{
       // todo!("Behövs name?");
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

    /// Resets bools that holds wether or not incoming signals and function code has been recived.
    fn reset_bools(&mut self) {
        self.has_val0 = false;
        self.has_val1 = false;
        self.has_signal= false;
    }


}

/// Mux implementing Unit trait.
impl Unit for Mux{

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word ){
        //println!("\t mux {} received data {} from {}",self.name, data, input_id);
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

    /// Receives signal from a Control, comes with ID to 
    /// specify which signal.
    /// 
    /// # Arguments
    /// 
    /// * `signal_id` - Id to know what type of signal is comming
    /// * `signal` - Bool which holds state of signal (high/low)
    /// 
    fn receive_signal(&mut self ,signal_id:u32, signal: bool){
        //println!("\t mux {} received signal {} with id {}",self.name, signal, signal_id);
        self.signal = signal;
        self.has_signal = true;
    }

    /// Checks if all data and signals needed has been received.
    /// If that is the case check which data should be forwarded to next Unit.
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












