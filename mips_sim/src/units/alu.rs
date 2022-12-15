
use std::ops::BitAnd;

use bitvec::prelude::*;
use std::ops::BitOr;
use std::ops::Not;
use crate::units::unit::*;


enum Operand{
    Add,
    And,
    Or,
    Slt,
}

pub struct ALU<'a> {

    data1 : Word,
    data2 : Word,
    
    has_data1: bool,
    has_data2: bool,

    mux_mem_to_reg :Option<&'a mut dyn Unit>,
    data_memory :Option<&'a mut dyn Unit>,
    ander :Option<&'a mut dyn Unit>,

    alu_signal0: bool,
    alu_signal1:bool,
    alu_signal2:bool,
    alu_signal3:bool,


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
            ander:None,

            alu_signal0: false,
            alu_signal1: false,
            alu_signal2: false,
            alu_signal3: false,


        }
    }

    //Execute unit with thread
    pub fn execute(&mut self){

        //Check if input should be inverted (bit 3 in control signal => a-invert, bit 2 => b-invert)
        if self.alu_signal3{
            self.data1= self.data1.to_bitvec().not();
        }
        if self.alu_signal2{
            self.data2= self.data2.to_bitvec().not();
        }
                
        //Check which operation should be done
        let operation = match self.alu_signal1{
            true =>{
                if self.alu_signal0{
                    println!("SLT");
                    Operand::Slt
                }else{
                    println!("ADD");
                    Operand::Add
                }
    
            }
                
            false =>{
                if self.alu_signal0{
                    println!("OR");
                    Operand::Or
                }else{
                    println!("AND");
                    Operand::And
                }
            }
        };
        
        //Process the data through the ALU in the same way as in hardware
        let (res,overflow,zero) = Self::process_data(operation, self.data1.to_bitvec(), self.data2.to_bitvec());

        //Send processed data to next units
        self.mux_mem_to_reg.as_mut().unwrap().receive(MUX_IN_0_ID, res.to_bitvec());
        self.data_memory.as_mut().unwrap().receive(DM_ADDR_ID, res.to_bitvec());
        self.ander.as_mut().unwrap().receive_signal(ZERO_SIGNAL, zero);
    }

   
    /// returns (result, overflow, zero)
    fn process_data(operation:Operand, data1:Word, data2:Word)->(Word,bool,bool){
        let mut res:Word = bitvec![u32, Lsb0; 0; 32];
        let mut overflow:bool = false;
        let mut word_a = data1.to_bitvec();
        let mut word_b = data2.to_bitvec();
        let mut carry_in:bool = false;
    

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
                        res.set(0, res_bit || carry_out);
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
    
        };

        //Check if result is zero with same method as in hardware
        let zero = Self::is_zero(res.to_bitvec());

        (res, overflow,zero)
    }  

        /// Set Functions
    pub fn set_mux_mem_to_reg(&mut self, mux: &mut dyn Unit){
        self.mux_mem_to_reg = Some(unsafe { std::mem::transmute(mux) });
    }

    pub fn set_data_mem_to_reg(&mut self, dm: &mut dyn Unit){
        self.data_memory = Some(unsafe { std::mem::transmute(dm) });
    }

    pub fn set_ander(&mut self, ander: &mut dyn Unit){
        self.ander = Some(unsafe { std::mem::transmute(ander) });
    }


    ///
    /// Adds bit a with bit b using carry_in
    /// Returns sum and carry out as tuple (sum, carry_out)
    /// 
    fn add_bits(carry_in:bool, a:bool, b:bool)->(bool, bool){
        let temp_carry1 = carry_in && a;
        let temp_carry2 = carry_in && b; 
        let temp_carry3 = b && a;

        let temp_sum1 = a && !b && carry_in;
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

    ///Check if word is equal to zero
    fn is_zero(word:Word)->bool{
        let mut zero:bool = false;
        for i in 0..32{
            zero = word[i] | zero;
        }
        zero = !zero;
        zero
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
        }else{
            //data came on undefined port
        }
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        if signal_id == ALU_CTRL0_SIGNAL{
            self.alu_signal0 = signal;
        }else if signal_id==ALU_CTRL1_SIGNAL{
            self.alu_signal1 = signal;
        }else if signal_id==ALU_CTRL2_SIGNAL{
            self.alu_signal2 = signal;
        }else if signal_id==ALU_CTRL3_SIGNAL{
            self.alu_signal3 = signal;
        }else{
            //Undefined signal
        }
    }
}


