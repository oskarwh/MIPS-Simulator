use crate::units::unit::*;
use std::sync::{Mutex, Arc};
use bitvec::prelude::*;

/// A MIPS simulator unit. Takes an Instruction Word from the memory,
/// divides it upp and sends the different parts to multiple Units.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.
/// 

/// InstructionMemory Struct
pub struct InstructionMemory {
    instructions: Vec<Word>,
    current_instruction: Word,
    current_address: u32,
    has_address: bool,

    reg : Option<Arc<Mutex<dyn Unit>>>,
    alu : Option<Arc<Mutex<dyn Unit>>>,
    sign_extend : Option<Arc<Mutex<dyn Unit>>>,
    alu_ctrl : Option<Arc<Mutex<dyn Unit>>>,
    control : Option<Arc<Mutex<dyn Unit>>>,
    concater: Option<Arc<Mutex<dyn Unit>>>,
    mux_regdst: Option<Arc<Mutex<dyn Unit>>>,
}

/// InstructionMemory Implementation
impl<'a> InstructionMemory{

    /// Returns a new InstructionMemory.
    ///
    /// # Arguments
    /// 
    /// * `instr` - Vector with all Instructions Words
    /// 
    /// # Returns
    ///
    /// * InstructionMemory
    ///
    pub fn new(instr: Vec<Word>) -> InstructionMemory{
        InstructionMemory{
            instructions:instr,
            current_instruction: bitvec![u32, Lsb0; 0; 32],
            current_address: 0,
            has_address: false,
            alu:None,
            reg: None,
            sign_extend: None,
            alu_ctrl: None,
            control: None,
            concater: None,
            mux_regdst:None,
        }
    }

    /// Set a Control that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `ctrl` - The Control that should be set
    ///
    pub fn set_control(&'a mut self, ctrl : Arc<Mutex<dyn Unit>>){
        self.control = Some(ctrl);
    }

    /// Set a RegisterFile that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `reg` - The RegisterFile that should be set
    ///
    pub fn set_reg(&'a mut self, reg : Arc<Mutex<dyn Unit>>){
        self.reg = Some(reg);
    }

    /// Set a SignExtend that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `sign_extend` - The SignExtend that should be set
    ///
    pub fn set_signextend(&'a mut self, sign_extend: Arc<Mutex<dyn Unit>>){
        self.sign_extend = Some(sign_extend);
    } 

    /// Set a AluControl that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `alu_ctrl` - The AluControl that should be set
    ///
    pub fn set_aluctrl(&'a mut self, alu_ctrl: Arc<Mutex<dyn Unit>>){
        self.alu_ctrl = Some(alu_ctrl);
    }

    /// Set a Concater that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `concater` - The Concater that should be set
    ///
    pub fn set_concater(&'a mut self, concater: Arc<Mutex<dyn Unit>>){
        self.concater = Some(concater);

    }

    /// Set which Mux that the 'InstructionMemory' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_regdst(&'a mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_regdst = Some(mux);

    }

    /// Set a ALU that the 'InstructionMemory' which is called on, should send signals to.
    /// 
    /// # Arguments
    ///
    /// * `alu` - The ALU that should be set
    ///
    pub fn set_alu(&'a mut self, alu: Arc<Mutex<dyn Unit>>){
        self.alu = Some(alu);

    }

    /// Shifts Bit Vector by a specified amount.
    /// 
    /// # Arguments
    ///
    /// * `word` - Bit Vector to be shifted
    /// * `shift` - Amount to shift 
    ///
    /// # Returns
    /// 
    /// * Word - Shifted Bit Vector
    fn shift_left(mut word: Word, shift: u32)->Word{
        word.shift_left(shift as usize);
        word
    }
}

/// InstructionMemory implementing Unit trait.
impl Unit for InstructionMemory{

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, address : Word){

        if input_id ==  IM_READ_ADDRESS_ID{
            //println!("\t Instruction-mem: recevied {}", address);
            self.current_address = address.to_bitvec().into_vec()[0];
            self.has_address = true;
        } else {
            //Message came on undefined input
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
    fn receive_signal(&mut self, _signal_id: u32, signal: bool) {
        // DO NOTHING
    }

    /// Checks if a index for the instruction memory is given.
    /// If that is the case divides the Instruction Word and sends the different parts to multiple Units.
    fn execute(&mut self) {
        if self.has_address {

            //Received address on read_address! Find corresponding instruction. Need to right shift 2 steps (divide by 4)
            self.current_address = self.current_address / 4;
            self.current_instruction = self.instructions[self.current_address as usize].to_bitvec();
            //println!("\t Instruction-mem: i have instruction  {}", self.current_instruction);
            //Send to concater, word will be shifted left (shift_right because of the way BitVec is designed)
            let mut borrow = self.current_instruction[0..26].to_bitvec();
            borrow.shift_right(2);
            self.concater.as_mut().unwrap().lock().unwrap().receive(CONC_IN_1_ID, borrow.to_bitvec() );

            //Send instruction to other units
            self.reg.as_mut().unwrap().lock().unwrap().receive(REG_READ_1_ID, Self::shift_left(self.current_instruction.to_bitvec(),21)[0..=4].to_bitvec());
            self.reg.as_mut().unwrap().lock().unwrap().receive(REG_READ_2_ID, Self::shift_left(self.current_instruction.to_bitvec(),16)[0..=4].to_bitvec());
            self.control.as_mut().unwrap().lock().unwrap().receive(CTRL_IN_ID,  Self::shift_left(self.current_instruction.to_bitvec(), 26)[0..=5].to_bitvec());
            self.control.as_mut().unwrap().lock().unwrap().receive(FUNCT_CONTROL,self.current_instruction[0..=5].to_bitvec());
            self.alu_ctrl.as_mut().unwrap().lock().unwrap().receive(ALU_CTRL_IN_ID,  self.current_instruction[0..=5].to_bitvec());
            self.sign_extend.as_mut().unwrap().lock().unwrap().receive(SE_IN_ID,  self.current_instruction[0..=15].to_bitvec());
            self.mux_regdst.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID,  Self::shift_left(self.current_instruction.to_bitvec(),16)[0..=4].to_bitvec());
            self.mux_regdst.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID,  Self::shift_left(self.current_instruction.to_bitvec(),11)[0..=4].to_bitvec());
            self.alu.as_mut().unwrap().lock().unwrap().receive(ALU_SHAMT_IN_ID,  Self::shift_left(self.current_instruction.to_bitvec(),6)[0..=4].to_bitvec());
            self.has_address = false;
        }
    }
}
