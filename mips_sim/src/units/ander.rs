
use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;
use std::sync::Mutex;
use std::sync::Arc;




pub struct Ander{

    zero_signal: bool,
    branch_signal : bool,

    mux_branch : Option<Arc<Mutex<dyn Unit>>>,

}


impl Ander{

    pub fn new() -> Ander{
        Ander{
            zero_signal: false,
            branch_signal: false,
            mux_branch: None,
        }
    }


    ///Execute unit with thread
    pub fn execute(&mut self){

        if self.zero_signal && self.branch_signal{
            //Append bits from instruction memory with address from PC+4
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive_signal(DEFAULT_SIGNAL,true);
        }else{
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive_signal(DEFAULT_SIGNAL,false);
        }
    }

    /// Set Functions
    pub fn set_mux_jump(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_branch = Some(unsafe { std::mem::transmute(mux) });
    }


}

impl Unit for Ander{

    fn receive(&mut self, input_id: u32, data : Word){
        //EMpty
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        if signal_id == ZERO_SIGNAL{
            self.zero_signal = signal;
        }else if signal_id == BRANCH_SIGNAL{
            self.branch_signal ==signal;
        }
    }
    
}