use bitvec::prelude::*;
use std::sync::{Mutex, Arc};
use crate::units::unit::*;


// Liftime paramters

pub struct AddUnit {

    addr : Word,
    sign_ext_instr: Word,
    has_instr: bool,
    has_addr : bool,

    mux_branch :Option<Arc<Mutex<dyn Unit>>>,
}


impl<'a> AddUnit{
    //Define MUX id's
    pub fn new() -> AddUnit{
        AddUnit{
            has_instr:false,
            has_addr:false,

            addr:bitvec![u32, Lsb0; 0; 32],
            sign_ext_instr: bitvec![u32, Lsb0; 0; 32],

            mux_branch: None,
        }
    }

    //Execute unit with thread
    pub fn execute(&'a mut self){

        if self.has_addr && self.has_instr{
            let (res,overflow) = Self::add(self.addr.to_bitvec(), self.sign_ext_instr.to_bitvec());
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, res);
            self.has_addr = false;
            self.has_instr = false;
        }
    }

        /// Set Functions
    pub fn set_mux_branch(&'a mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_branch = Some(mux);
    }


    fn add(data1:Word, data2:Word)->(Word,bool){
        let mut res:Word = bitvec![u32, Lsb0; 0; 32];
        let mut overflow:bool = false;
        let word_a = data1.to_bitvec();
        let word_b = data2.to_bitvec();
        let mut carry_in:bool = false;
        let mut carry_out:bool = false;
        let mut res_bit:bool;

        for i in 0..32{
            (res_bit, carry_out) = Self::add_bits(carry_in, word_a[i], word_b[i]);
            carry_in = carry_out;
            
            if i == 31{
                overflow = Self::detect_overflow(res_bit, word_a[i], word_b[i]);
            }
            res.set(i, res_bit);
        }
        (res,overflow)
    
    }

    ///
    /// Adds bit a with bit b using carry_in
    /// Returns sum and carry out as tuple (sum, carry_out)
    /// 
    fn add_bits(carry_in:bool, a:bool, b:bool)->(bool, bool){
        let temp_carry1 = carry_in && a;
        let temp_carry2 = carry_in && b; 
        let temp_carry3 = b && a;

        let temp_sum1 = a && !b && !carry_in;
        let temp_sum2 = !a && b && !carry_in; 
        let temp_sum3 = !a && !b && carry_in;
        let temp_sum4 = a && b && carry_in;


        let carry_out = temp_carry1 || temp_carry2 || temp_carry3;
        let sum = temp_sum1 || temp_sum2 || temp_sum3 || temp_sum4;

        (sum, carry_out)
    }

    ///Detects overflow during addition on two words
    /// Takes msb for both words (a,b) current carry, the current sum and b_invert (to know if we are doing subtraction)
    fn detect_overflow(sum:bool , a:bool, b:bool)->bool{
        //Check for overflow, follows fig 3.2 in the course book
        (!a && !b && sum || a && b && !sum)
    }


    
}

impl<'a> Unit for AddUnit {
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == ADD_IN_1_ID{
            self.addr = data;
            self.has_addr = true;
        }else if input_id == ADD_IN_2_ID{
            self.sign_ext_instr = data;
            self.has_instr = true;
        }
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        // DO NOTHING
    }


    
}



