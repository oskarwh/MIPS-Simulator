
use bitvec::prelude::*;
use crate::units::unit::*;


// Liftime paramters

pub struct AddUnit<'a> {

    addr : Word,
    sign_ext_instr: Word,
    has_instr: bool,
    has_addr : bool,

    mux_branch :&'a mut dyn Unit,
}

pub struct AddUnitBuilder<'a> {

    mux_branch :Option<&'a mut dyn Unit>,
}

impl AddUnitBuilder<'_>{
    pub fn new()->AddUnitBuilder<'static>{
        AddUnitBuilder{
            mux_branch: None,
        }
    }

    /// Set Functions
    pub fn set_mux_branch<'a>(&mut self, mux: &'a mut dyn Unit){
        self.mux_branch = Some(mux);
    }
    
    //Consumes itself and creates an AddUnit
    fn build(self)->AddUnit<'static>{
        AddUnit{
            has_instr:false,
            has_addr:false,

            addr:bitvec![u32, Lsb0; 0; 32],
            sign_ext_instr: bitvec![u32, Lsb0; 0; 32],

            mux_branch: self.mux_branch.expect("Need to set mux_branch in AddUnit builder before it can be built"),
        }
    }

}

impl AddUnit<'_>{


    //Execute unit with thread
    pub fn execute(&mut self){

        if self.has_addr && self.has_instr{
            let res = Self::add(self.addr.to_bitvec(), self.sign_ext_instr.to_bitvec());
            self.mux_branch.receive(MUX_IN_1_ID, res);
        }
    }


    fn add(word1 : Word, word2 : Word) -> Word {
        let num1 = word1.into_vec()[0];
        let num2 = word2.into_vec()[0];

        let res = num1 + num1;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    
}

impl Unit for AddUnit<'_>  {
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == ADD_IN_1_ID{
            self.addr = data;
            self.has_addr = true;
        }else if input_id == ADD_IN_2_ID{
            self.sign_ext_instr = data;
            self.has_instr = true;
        }
    }

    fn receive_signal(&mut self,signal_id:u32) {
        // DO NOTHING
    }
}





