
use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;
use std::sync::Arc;
use std::sync::Mutex;




pub struct Concater {

    addr : Word,
    instr : Word,
    has_instr: bool,
    has_addr : bool,

    mux_jump : Option<Arc<Mutex<dyn Unit>>>,

}


impl Concater{

    pub fn new() -> Concater{
        Concater{
            has_addr:false,
            has_instr:false,
            addr: bitvec![u32, Lsb0; 0; 32],
            instr: bitvec![u32, Lsb0; 0; 32],
            mux_jump: None,
        }
    }




    /// Set Functions
    pub fn set_mux_jump(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_jump = Some(mux);
    }


}

impl Unit for Concater{

    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == CONC_IN_1_ID{
            self.instr = data;
            self.has_instr = true;
        }else if input_id == CONC_IN_2_ID{
            self.addr = data;
            self.has_addr = true;
        }
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        // DO NOTHING
    }

    ///Execute unit with thread
    fn execute(&mut self){

        if self.has_addr && self.has_instr{
            //Append bits from instruction memory with address from PC+4
            self.addr.append(&mut self.instr);
            self.mux_jump.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, self.addr.to_bitvec());
            self.has_addr = false;
            self.has_instr = false;
        }
    }
    
}




