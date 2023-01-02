use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Arc;
use std::sync::Mutex;

/// A MIPS simulator unit. Will concat two incoming Bit Vectors.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// Concater Struct
pub struct Concater {

    addr : Word,
    instr : Word,
    has_instr: bool,
    has_addr : bool,

    mux_jump : Option<Arc<Mutex<dyn Unit>>>,

}

/// Concater Implementation
impl Concater{

    /// Returns a new Concater.
    ///
    /// # Returns
    ///
    /// * Concater
    ///
    pub fn new() -> Concater{
        Concater{
            has_addr:false,
            has_instr:false,
            addr: bitvec![u32, Lsb0; 0; 32],
            instr: bitvec![u32, Lsb0; 0; 32],
            mux_jump: None,
        }
    }

    /// Set which Mux that the 'Concater' which is called on, should send concated Bit Vector to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_jump(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_jump = Some(mux);
    }


}

/// Concater implementing Unit trait.
impl Unit for Concater{

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == CONC_IN_1_ID{
            self.instr = data;
            self.has_instr = true;
        }else if input_id == CONC_IN_2_ID{
            self.addr = data;
            self.has_addr = true;
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
        // DO NOTHING
    }

    /// Checks if all signals and data needed has been received.
    /// If that is the case the concater will append one the first Bit Vector to 
    /// the other and send the result to the choose Mux.
    fn execute(&mut self){

        if self.has_addr && self.has_instr{
            //Append bits from instruction memory with address from PC+4
            self.instr.append(&mut self.addr);
            //println!("\tConcater sending: {}", self.instr);
            self.mux_jump.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, self.instr.to_bitvec());
            self.has_addr = false;
            self.has_instr = false;
        }
    }
    
}




