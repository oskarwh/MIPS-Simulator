use bitvec::prelude::*;
use std::{sync::Mutex, ops::Deref};

pub type Word = BitVec<u32, Lsb0>;
pub trait Unit: Send + Sync {
    fn receive(&mut self, input_id : u32, data :Word);
    fn receive_signal(&mut self ,signal_id:u32, signal: bool);
    fn execute(&mut self);
}

pub struct EmptyUnit<'a>{
    name:&'a str,
}



impl<'a> EmptyUnit<'a>{
    pub fn new(name:&'a str)->EmptyUnit<'a>{
        EmptyUnit{
            name
        }
    }
}

impl<'a> Unit for EmptyUnit<'a>{
    fn receive(&mut self, input_id : u32, data :Word){
        println!("Empty {} received at port {}: {}",self.name, input_id, data);
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
       
    }

    fn execute(&mut self) {
       
    }
}
    

pub const PC_IN_ID :u32 = 0;

pub const OP_CONTROL: u32 = 0;
pub const FUNCT_CONTROL: u32 = 0;

pub const IM_READ_ADDRESS_ID:u32  = 0;

pub const REG_READ_1_ID:u32  = 0;
pub const REG_READ_2_ID :u32 = 1;
pub const REG_WRITE_DATA_ID:u32  = 2;
pub const REG_WRITE_REG_ID :u32 = 3;

pub const CTRL_IN_ID:u32  = 0;

pub const ALU_CTRL_IN_ID :u32 = 0;

pub const SE_IN_ID:u32  = 0;

pub const AC_IN_ID:u32  = 0;

pub const ALU_IN_1_ID:u32  = 0;
pub const ALU_IN_2_ID :u32 = 1;
pub const ALU_SHAMT_IN_ID :u32 = 2;

pub const DM_ADDR_ID:u32  = 0;
pub const DM_DATA_ID:u32  = 1;

pub const MUX_IN_0_ID:u32  = 0;
pub const MUX_IN_1_ID:u32  = 1;

pub const CONC_IN_1_ID:u32  = 0;
pub const CONC_IN_2_ID:u32  = 1;

pub const ADD_IN_1_ID :u32 = 0;
pub const ADD_IN_2_ID:u32  = 1;

//Signals for branching
pub const ZERO_SIGNAL:u32 = 0;
pub const BRANCH_SIGNAL:u32 = 1;
// Define control signals for data memory, since it has two signals
pub const MEM_WRITE_SIGNAL:u32  = 0;
pub const MEM_READ_SIGNAL:u32  = 1;
// Define control signals for alu control op code, also tow signals
pub const ALU_OP0_SIGNAL:u32 = 0;
pub const ALU_OP1_SIGNAL:u32  = 1;
pub const ALU_OP2_SIGNAL:u32  = 2;
// Define default signal const for all components with just one signal.
pub const DEFAULT_SIGNAL: u32  = 0;     

// Define ALU control signals
pub const ALU_CTRL0_SIGNAL:u32  = 0;
pub const ALU_CTRL1_SIGNAL:u32  = 1;
pub const ALU_CTRL2_SIGNAL:u32  = 2;
pub const ALU_CTRL3_SIGNAL:u32  = 3;
pub const ALU_CTRL4_SIGNAL:u32  = 4;















