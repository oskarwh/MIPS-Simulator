
use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;




struct Concater<'a> {

    addr : Word,
    instr : Word,
    has_instr: bool,
    has_addr : bool,

    mux_jump : &'a dyn Unit,

}


impl Concater<'_>{

    pub fn new() -> Concater<'static>{
        Concater{
            has_addr:false,
            has_instr:false,
            addr: bitvec![u32, Lsb0; 0; 32],
            instr: bitvec![u32, Lsb0; 0; 32],
            mux_jump: &EmptyUnit{},
        }
    }


    ///Execute unit with thread
    pub fn execute(&self){

        if self.has_addr && self.has_instr{
            //Append bits from instruction memory with address from PC+4
            self.addr.append(&mut self.instr);
            self.mux_jump.receive(MUX_IN_1_ID, self.addr.to_bitvec());
        }
    }

    /// Set Functions
    pub fn set_mux_jump(&self, mux: &impl Unit){
        self.mux_jump = mux;
    }


}

impl Unit for Concater<'_>{

    fn receive(&self, input_id: u32, data : Word){
        if input_id == CONC_IN_1_ID{
            self.instr = data;
            self.has_instr = true;
        }else if input_id == CONC_IN_2_ID{
            self.addr = data;
            self.has_addr = true;
        }
    }

    fn receive_signal(&self ,signal_id:u32) {
        // DO NOTHING
    }
    
}




