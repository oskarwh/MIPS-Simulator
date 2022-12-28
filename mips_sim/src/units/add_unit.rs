use bitvec::prelude::*;
use std::sync::{Mutex, Arc};
use crate::units::unit::*;

/// A MIPS simulator unit. Will add the current PC adress and the offset
/// given with a BEQ instruction and compute the adress to jump to.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// AddUnit 
pub struct AddUnit {

    addr : Word,
    sign_ext_instr: Word,
    has_instr: bool,
    has_addr : bool,

    mux_branch :Option<Arc<Mutex<dyn Unit>>>,
}

impl AddUnit{
    /// Returns a new AddUnit.
    ///
    /// # Returns
    ///
    /// *  AddUnit
    ///
    pub fn new() -> AddUnit{
        AddUnit{
            has_instr:false,
            has_addr:false,

            addr:bitvec![u32, Lsb0; 0; 32],
            sign_ext_instr: bitvec![u32, Lsb0; 0; 32],

            mux_branch: None,
        }
    }



    /// Set which Mux that the 'AddUnit' which is called on, should send next address to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_branch(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_branch = Some(mux);
    }

    /// Adds two Bit Vectors together and returns the new Bit Vector
    /// and a boolean signaling if there were any overflow.
    /// 
    /// # Arguments
    ///
    /// * `data1` - First Bit Vector
    /// * `data2` - Second Bit Vector
    /// 
    /// # Returns
    ///
    /// *  (Word, bool) - Word holds the new Bit Vector and the bool if there were overflow.
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

 
    /// Adds two bits together using a carry in from previous add and 
    /// returning a the sum and carry out.
    /// 
    /// # Arguments
    ///
    /// * `carry_in` - Overflow from previous bit add
    /// * `a` - First bit
    /// * `b` - Second bit
    /// 
    /// # Returns
    ///
    /// *  (bool, bool) - First bool holds sum from add, second holds any overflow.
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

    /// Detects overflow during addition on two Bit Vectors.
    /// 
    /// # Arguments
    /// 
    /// * `sum` - Carry in
    /// * `a`- Most significant bit from first word
    /// * `b`- Most significant bit from second word
    /// 
    ///  # Result
    /// 
    /// * bool - True if overflow was detected otherwise false.
    fn detect_overflow(sum:bool , a:bool, b:bool)->bool{
        //Check for overflow, follows fig 3.2 in the course book
        (!a && !b && sum || a && b && !sum)
    }


    
}

/// AddUnit implementing Unit trait.
impl<'a> Unit for AddUnit {
    /// Receives data from some Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
       // println!("\t Add unit: received {} from {}", data, input_id);
        if input_id == ADD_IN_1_ID{
            self.addr = data;
            self.has_addr = true;
        }else if input_id == ADD_IN_2_ID{
            self.sign_ext_instr = data;
            self.has_instr = true;
        }
    }

    /// Receives signal from some Control, comes with ID to 
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

    /// Checks if all data and signals needed has be recived.
    /// If that is the case add the incoming Bit Vectors together and 
    /// send the result to given Mux.
    /// 
    fn execute(&mut self){

        if self.has_addr && self.has_instr{
            let (res,overflow) = Self::add(self.addr.to_bitvec(), self.sign_ext_instr.to_bitvec());
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, res);
            self.has_addr = false;
            self.has_instr = false;
        }
    }


    
}



