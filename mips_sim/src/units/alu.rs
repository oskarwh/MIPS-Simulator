
use bitvec::prelude::*;
use crate::units::unit::*;


enum Operand{
    Add,
    Sub,
    And,
    Or,
    Sll,

}

pub struct ALU<'a> {

    data1 : Word,
    data2 : Word,
    
    has_data1: bool,
    has_data2: bool,

    mux_mem_to_reg :Option<&'a mut dyn Unit>,
    data_memory :Option<&'a mut dyn Unit>,

    alu_signal1: bool,
    alu_signal2:bool,
    alu_signal3:bool,
    alu_signal4:bool,
}


impl ALU<'_>{
    //Define MUX id's
    pub fn new() -> ALU<'static>{
        ALU { 
            data1: bitvec![u32, Lsb0; 0; 32],
            data2: bitvec![u32, Lsb0; 0; 32],

            has_data1: false, 
            has_data2: false, 

            mux_mem_to_reg: None, 
            data_memory: None,
        }
    }

    //Execute unit with thread
    pub fn execute(&mut self){

        if self.has_data1 && self.has_data2{
            let res = Self::add(self.addr.to_bitvec(), self.sign_ext_instr.to_bitvec());
            self.mux_branch.as_mut().unwrap().receive(MUX_IN_1_ID, res);
        }
    }

        /// Set Functions
    pub fn set_mux_branch(&mut self, mux: &mut dyn Unit){
        self.mux_branch = Some(unsafe { std::mem::transmute(mux) });
    }


    fn add(word1 : Word, word2 : Word) -> Word {
        let num1 = word1.into_vec()[0];
        let num2 = word2.into_vec()[0];

        let res = num1 + num2;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    
}

impl Unit for ALU<'_>  {
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == ALU_IN_1_ID{
            self.data1 = data;
            self.has_data1 = true;
        }else if input_id == ALU_IN_2_ID{
            self.data2 = data;
            self.has_data2 = true;
        }
    }

    fn receive_signal(&mut self ,signal_id:u32) {
        if signal_id == ALU_CTRL0_SIGNAL{

        }
    }
}


