use bitvec::prelude::*;

/// Max words in data memory
pub const MAX_WORDS: usize = 250;

/// Alias for BitVec of size 32.
pub type Word = BitVec<u32, Lsb0>;

/// Unit Trait
pub trait Unit: Send + Sync {
    
    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id : u32, data :Word);

    /// Receives signal from a Control, comes with ID to 
    /// specify which signal.
    /// 
    /// # Arguments
    /// 
    /// * `signal_id` - Id to know what type of signal is comming
    /// * `signal` - Bool which holds state of signal (high/low)
    /// 
    fn receive_signal(&mut self ,signal_id:u32, signal: bool);

    /// Executes specific Unit behvaiour.
    fn execute(&mut self);
}
 
/// Constans for data types.
pub const PC_IN_ID :u32 = 0;

pub const OP_CONTROL: u32 = 0;
pub const FUNCT_CONTROL: u32 = 1;

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


/// Constans for signal types.
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















