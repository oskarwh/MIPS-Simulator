
use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;
use std::sync::Mutex;
use std::sync::Arc;




pub struct Ander{

    zero_signal: bool,
    branch_signal : bool,

    has_zero_signal: bool,
    has_branch_signal :bool,

    mux_branch : Option<Arc<Mutex<dyn Unit>>>,

}


impl Ander{

    pub fn new() -> Ander{
        Ander{
            zero_signal: false,
            branch_signal: false,
            mux_branch: None,
            has_zero_signal: false,
            has_branch_signal: false,
        }
    }


    /// Set Functions
    pub fn set_mux_branch(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_branch = Some(unsafe { std::mem::transmute(mux) });
    }


}

impl Unit for Ander{

    fn receive(&mut self, input_id: u32, data : Word){
        //EMpty
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        println!("\t Ander received signal {} from {}", signal, signal_id);
        if signal_id == ZERO_SIGNAL{
            self.zero_signal = signal;
            self.has_zero_signal = true;
        }else if signal_id == BRANCH_SIGNAL{
            self.branch_signal ==signal;
            self.has_branch_signal = true;
        }
    }
    
    

    ///Execute unit with thread
    fn execute(&mut self){

        if self.has_zero_signal && self.has_branch_signal{
            //Append bits from instruction memory with address from PC+4
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive_signal(DEFAULT_SIGNAL,self.branch_signal && self.zero_signal);
            self.has_branch_signal = false;
            self.has_zero_signal = false;
        }
    }
}