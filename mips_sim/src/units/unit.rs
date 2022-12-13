use bitvec::prelude::*;

pub type Word = BitVec<u32, Lsb0>;
pub trait Unit{
    

    fn receive(&self, input_id : u32, data :Word);
    fn receive_signal(&self ,signal_id:u32);
}

pub struct EmptyUnit{

}

impl Unit for EmptyUnit{
    fn receive(&self, input_id : u32, data :Word){
        println!("Empty Unit received data: data = {}", data);
    }

    fn receive_signal(&self ,signal_id:u32) {
        todo!()
    }


}
    

pub const PC_IN_ID :u32 = 0;

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

pub const DM_ADDR_ID:u32  = 0;
pub const DM_DATA_ID:u32  = 1;

pub const MUX_IN_0_ID:u32  = 0;
pub const MUX_IN_1_ID:u32  = 1;

pub const CONC_IN_1_ID:u32  = 0;
pub const CONC_IN_2_ID:u32  = 1;

pub const ADD_IN_1_ID :u32 = 0;
pub const ADD_IN_2_ID:u32  = 1;


// Define control signals for data memory, since it has two signals
pub const MEM_WRITE_SIGNAL:u32  = 0;
pub const MEM_READ_SIGNAL:u32  = 1;
// Define control signals for alu control op code, also tow signals
pub const ALU_OP0_SIGNAL :u32 = 0;
pub const ALU_OP1_SIGNAL:u32  = 1;
// Define default signal const for all components with just one signal.
pub const DEFAULT_SIGNAL:u32  = 0;    

// Define ALU control signals
pub const ALU_CTRL0_SIGNAL:u32  = 0;
pub const ALU_CTRL1_SIGNAL:u32  = 1;
pub const ALU_CTRL2_SIGNAL:u32  = 2;
pub const ALU_CTRL3_SIGNAL:u32  = 3;
