mod unit;
mod mux;

use bitvec::prelude::*;
use unit::Word;
use unit::Unit;
use unit::EmptyUnit;


// Liftime paramters

struct AddUnit<'a> {

    addr : Word,
    sign_ext_instr: Word,
    has_instr: bool,
    has_addr : bool,

    mux_branch :&'a dyn Unit,
}


impl AddUnit<'_>{
    //Define MUX id's
    pub fn new() -> AddUnit<'static>{
        AddUnit{
            has_instr:false,
            has_addr:false,

            addr:bitvec![u32, Lsb0; 0; 32],
            sign_ext_instr: bitvec![u32, Lsb0; 0; 32],

            mux_branch: &EmptyUnit{},
        }
    }

    //Execute unit with thread
    pub fn execute(&self){

        if self.has_addr && self.has_instr{
            let res = Self::add(self.addr, self.sign_ext_instr);
            self.mux_branch.receive(unit::MUX_IN_1_ID, res);
        }
    }

    /// Set Functions
    pub fn set_mux_branch(&self, mux: &impl Unit){
        self.mux_branch = mux;
    }


    fn add(word1 : Word, word2 : Word) -> Word {
        let num1 = word1.into_vec()[0];
        let num2 = word2.into_vec()[0];

        let res = num1 + num1;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    
}

impl Unit for AddUnit<'_>  {
    fn receive(&self, input_id: u32, data : Word){
        if input_id == unit::ADD_IN_1_ID{
            self.addr = data;
            self.has_addr = true;
        }else if input_id == unit::ADD_IN_2_ID{
            self.sign_ext_instr = data;
            self.has_instr = true;
        }
    }

    fn receive_signal(&self ,signal_id:u32) {
        // DO NOTHING
    }
}





