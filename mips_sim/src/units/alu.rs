use std::ops::BitAnd;
use std::sync::Arc;
use std::sync::Mutex;

use bitvec::prelude::*;
use std::ops::BitOr;
use std::ops::Not;
use crate::units::unit::*;

/// A MIPS simulator unit. The ALU or Arithmetic and Logic Unit perfroms different operation on the received data based 
/// on which signals it recives from the ALU Control. 
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// Enum for ALU operations
enum Operand{
    Add,
    And,
    Or,
    Slt,
    Srl,
    Sra,
}

/// ALU Struct
pub struct ALU {

    data1 : Word,
    data2 : Word,
    shamt: Word,
    
    has_data1: bool,
    has_data2: bool,
    has_shamt: bool,

    mux_mem_to_reg :Option<Arc<Mutex<dyn Unit>>>,
    data_memory :Option<Arc<Mutex<dyn Unit>>>,
    ander :Option<Arc<Mutex<dyn Unit>>>,

    alu_signal0: bool,
    alu_signal1:bool,
    alu_signal2:bool,
    alu_signal3:bool,
    alu_signal4:bool,

    has_alu_signal0: bool,
    has_alu_signal1: bool,
    has_alu_signal2: bool,
    has_alu_signal3: bool,
    has_alu_signal4:bool,


}

/// ALU Implementation
impl ALU {
    /// Returns a new ALU.
    ///
    /// # Returns
    ///
    /// * ALU
    ///
    pub fn new() -> ALU {
        ALU { 
            data1: bitvec![u32, Lsb0; 0; 32],
            data2: bitvec![u32, Lsb0; 0; 32],
            shamt: bitvec![u32, Lsb0; 0; 32],

            has_data1: false, 
            has_data2: false, 
            has_shamt: false,

            mux_mem_to_reg: None, 
            data_memory: None,
            ander:None,

            has_alu_signal0: false,
            has_alu_signal1: false,
            has_alu_signal2: false,
            has_alu_signal3: false,
            has_alu_signal4: false,

            alu_signal0: false,
            alu_signal1: false,
            alu_signal2: false,
            alu_signal3: false,
            alu_signal4 : false,
            
            
            


        }
    }


    /// Processes the two incoming Bit Vectors using a operation chosen as a argument.
    /// Subtraction uses Add operation with a carry in and inverted Bit Vectors.
    /// 
    /// # Arguments
    ///
    /// * `operation` - The operation to perform
    /// * `data1` - First Bit Vector
    /// * `data2` - Second Bit Vector
    /// * `shamt` - Shift amount
    /// * `carry_in` - Bool that holds carry_in
    /// 
    /// # Returns
    /// 
    /// * (Word, bool, bool) - Computed result, Overflow if there is any, Boolean to check if Bit Vectors are equal
    ///
    fn process_data(operation:Operand, data1:Word, data2:Word, shamt:u32,mut carry_in:bool)->(Word,bool,bool){
        let mut res:Word = bitvec![u32, Lsb0; 0; 32];
        let mut overflow:bool = false;
        let mut word_a = data1.to_bitvec();
        let mut word_b = data2.to_bitvec();
        
    

        let res = match operation{
            Operand::Add =>{
                let mut carry_out:bool = false;
                let mut res_bit:bool = false;
                for i in 0..32{
                    (res_bit, carry_out) = Self::add_bits(carry_in, word_a[i], word_b[i]);
                    carry_in = carry_out;
                    
                    if i == 31{
                        overflow = Self::detect_overflow(res_bit, word_a[i], word_b[i]);
                    }
                    res.set(i, res_bit);
                }
                res
            }
    
            Operand::Slt =>{
                let mut carry_out:bool = false;
                let mut res_bit:bool = false;
                
                for i in 0..32{
                    (res_bit, carry_out) = Self::add_bits(carry_in, word_a[i], word_b[i]);
                    carry_in = carry_out;
    
                    if i == 31{
                        overflow = Self::detect_overflow(res_bit, word_a[i], word_b[i]);
                        res.set(0, res_bit);
                    }else{
                        res.set(i, false);
                    }   
                }
                res
            }
    
            Operand::And =>{
                word_a.bitand(word_b)
            }
    
            Operand::Or => {
                word_a.bitor(word_b)
            }

            Operand::Sra => {
                let old_bits = 32 - shamt as usize;
                // Check if word is negative or positve
                if word_b[31] {
                    // Word is negative so make bitvec with ones at each new bit from shift
                    let mut ones = bitvec![u32, Lsb0; 1; shamt as usize];
                    // Make bitvec with zeros for old bits
                    let mut zeros = bitvec![u32, Lsb0; 0; old_bits];

                    zeros.append(&mut ones);

                    word_b.shift_left(shamt as usize);
                    word_b= word_b.bitor(zeros);
                } else {
                    word_b.shift_left(shamt as usize);
                }
                word_b
            }

            Operand::Srl => {
                word_b.shift_left(shamt as usize);
                word_b
            }
    
        };

        //Check if result is zero with same method as in hardware
        
        let zero = Self::is_zero(res.to_bitvec());
        (res, overflow,zero)
    }  


    /// Set which Mux that the 'ALU' which is called on, should send the computed result to.
    /// Which in turn controls what should be sent back to Register File.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_mem_to_reg(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_mem_to_reg = Some(mux);
    }

    /// Set a Data Memory that the 'AluControl' which is called on, should send the computed result to.
    /// 
    /// # Arguments
    ///
    /// * `dm` - The Data Memory that should be set
    ///
    pub fn set_data_mem_to_reg(&mut self, dm: Arc<Mutex<dyn Unit>>){
        self.data_memory = Some(dm);
    }

    /// Set a "Ander" that the 'AluControl' which is called on, should send zero signal to
    /// if a BEQ instruction is true.
    /// 
    /// # Arguments
    ///
    /// * `ander` - The "Ander" that should be set
    ///
    pub fn set_ander(&mut self, ander: Arc<Mutex<dyn Unit>>){
        self.ander = Some(ander);
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

    /// Check if Bit Vector is equal to zero.
    /// 
    /// # Arguments
    /// 
    /// * `word` - Bit Vector to check
    /// 
    /// # Returns
    /// 
    /// * bool - True if Bit Vector is Zero otherwise false.
    fn is_zero(word:Word)->bool{
        let mut zero:bool = false;
        for i in 0..32{
            zero = word[i] | zero;
        }
        zero = !zero;
        zero
    }

    /// Resets bools that holds wether or not incoming signals and function code has been recived.
    fn reset_bools(&mut self) {
        self.has_data1 = false;
        self.has_data2 = false;
        self.has_alu_signal0= false;
        self.has_alu_signal1= false;
        self.has_alu_signal2= false;
        self.has_alu_signal3=false;
    }

}

/// ALU implementing Unit trait.
impl Unit for ALU  {

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == ALU_IN_1_ID{
            self.data1 = data;
            self.has_data1 = true;
        }else if input_id == ALU_IN_2_ID{
            self.data2 = data;
            self.has_data2 = true;
        }else if input_id == ALU_SHAMT_IN_ID{
            self.shamt = data;
            self.has_shamt = true;
        }else{
            //data came on undefined port
        }
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
        if signal_id == ALU_CTRL0_SIGNAL{
            self.alu_signal0 = signal;
            self.has_alu_signal0 =true;
        }else if signal_id==ALU_CTRL1_SIGNAL{
            self.alu_signal1 = signal;
            self.has_alu_signal1 =true;
        }else if signal_id==ALU_CTRL2_SIGNAL{
            self.alu_signal2 = signal;
            self.has_alu_signal2 = true;
        }else if signal_id==ALU_CTRL3_SIGNAL{
            self.alu_signal3 = signal;
            self.has_alu_signal3 = true;
        }else if signal_id==ALU_CTRL4_SIGNAL{
            self.alu_signal4 = signal;
            self.has_alu_signal4 = true;
        }else{
            //Undefined signal
        }
    }

    /// Checks if all data and signals needed has been received.
    /// If that is the case checks based on incoming signals which operation to 
    /// perform on incoming data and perfroms correct operation.
    fn execute(&mut self){
        if self.has_data1 && self.has_data2 && self.has_alu_signal0 &&  self.has_alu_signal1 && self.has_alu_signal2 
                && self.has_alu_signal3 && self.has_alu_signal4 && self.has_shamt{

            //Need to set carry_in here because alu_signal2 choses if its true or false
            let mut carry_in:bool = false;
            //Check if input should be inverted (bit 3 in control signal => a-invert, bit 2 => b-invert)
            if self.alu_signal3{
                self.data1= self.data1.to_bitvec().not();
            }
            if self.alu_signal2{
                carry_in = true;
                self.data2= self.data2.to_bitvec().not();
            }
                    
            //Check which operation should be done
            let operation = match self.alu_signal4{
                false=>{
                    match self.alu_signal1{
                        true =>{
                            if self.alu_signal0{
                                Operand::Slt
                            }else{
                                Operand::Add
                            }
        
                        }
                        false =>{
                            if self.alu_signal0{
                                Operand::Or
                            }else{
                                Operand::And
                            }
                        }
                    }
                   
                }
                true=>{
                    if self.alu_signal0{
                        Operand::Sra
                    }else{
                        Operand::Srl
                    }
                }
                
            };
            
            //Process the data through the ALU in the same way as in hardware
            let (res,overflow,zero) = Self::process_data(operation, self.data1.to_bitvec(), self.data2.to_bitvec(), self.shamt.to_bitvec().into_vec()[0], carry_in);

            //Send processed data to next units
            self.mux_mem_to_reg.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID, res.to_bitvec());
            self.data_memory.as_mut().unwrap().lock().unwrap().receive(DM_ADDR_ID, res.to_bitvec());
            self.ander.as_mut().unwrap().lock().unwrap().receive_signal(ZERO_SIGNAL, zero);

            self.reset_bools();
        }
       
    }
    
}


