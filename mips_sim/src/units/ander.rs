use crate::units::unit::*;
use std::sync::Mutex;
use std::sync::Arc;

/// A MIPS simulator unit. Does an And operation on two incoming signals, 
/// one from the ALU and one from the Controller. If both are true Ander will send
/// a true signal otherwise false.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// Ander Struct
pub struct Ander{

    zero_signal: bool,
    branch_signal : bool,

    has_zero_signal: bool,
    has_branch_signal :bool,

    mux_branch : Option<Arc<Mutex<dyn Unit>>>,

}

// Ander Implementation
impl Ander{
    
    /// Returns a new Ander.
    ///
    /// # Returns
    ///
    /// * Ander
    ///
    pub fn new() -> Ander{
        Ander{
            zero_signal: false,
            branch_signal: false,
            mux_branch: None,
            has_zero_signal: false,
            has_branch_signal: false,
        }
    }


    /// Set which Mux that the 'AddUnit' which is called on, should send signal to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_branch(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_branch = Some(mux);
    }


}

/// AddUnit implementing Unit trait.
impl Unit for Ander{

    /// Receives data from some Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        //EMpty
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
        if signal_id == ZERO_SIGNAL{
            self.zero_signal = signal;
            self.has_zero_signal = true;
        }else if signal_id == BRANCH_SIGNAL{
            self.branch_signal =signal;
            self.has_branch_signal = true;
        }
    }
    
    /// Checks if all signals needed has been received.
    /// If that is the case the Ander will send correct signal to choosen Mux.
    fn execute(&mut self){

        if self.has_zero_signal && self.has_branch_signal{
            //Append bits from instruction memory with address from PC+4
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive_signal(DEFAULT_SIGNAL,self.branch_signal && self.zero_signal);
            self.has_branch_signal = false;
            self.has_zero_signal = false;
        }
    }
}