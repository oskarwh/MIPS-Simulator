use crate::units::unit::*;

use std::sync::{Mutex, Arc};
use std::boxed::Box;

use bitvec::prelude::*;

pub struct InstructionMemory {
    instructions: Vec<Word>,
    current_instruction: Word,
    current_address: u32,
    has_address: bool,

    reg : Option<Arc<Mutex<dyn Unit>>>,
    sign_extend : Option<Arc<Mutex<dyn Unit>>>,
    alu_ctrl : Option<Arc<Mutex<dyn Unit>>>,
    control : Option<Arc<Mutex<dyn Unit>>>,
    concater: Option<Arc<Mutex<dyn Unit>>>,
}


impl<'a> InstructionMemory{

    pub fn new(instr: Vec<Word>) -> InstructionMemory{
        InstructionMemory{
            instructions:instr,
            current_instruction: bitvec![u32, Lsb0; 0; 32],
            current_address: 0,
            has_address: false,

            reg: None,
            sign_extend: None,
            alu_ctrl: None,
            control: None,
            concater: None,
        }
    }

    ///Execute unit with thread
    pub fn execute(&mut self) {
        if self.has_address {
            println!("Has address");
            //Received address on read_address! Find corresponding instruction. Need to right shift 2 steps (divide by 4)
            self.current_address = self.current_address / 4;
            self.current_instruction = self.instructions[self.current_address as usize].to_bitvec();

            //Send to concater, word will be shifted left (shift_right because of the way BitVec is designed)
            let mut borrow = self.current_instruction[0..26].to_bitvec();
            borrow.shift_right(2);

            self.concater.as_mut().unwrap().lock().unwrap().receive(CONC_IN_1_ID, borrow.to_bitvec() );

            //Send instruction to other units
            self.reg.as_mut().unwrap().lock().unwrap().receive(REG_READ_1_ID, self.current_instruction[21..26].to_bitvec());
            self.reg.as_mut().unwrap().lock().unwrap().receive(REG_READ_2_ID, self.current_instruction[16..21].to_bitvec());
            self.control.as_mut().unwrap().lock().unwrap().receive(CTRL_IN_ID, self.current_instruction[26..32].to_bitvec());
            self.alu_ctrl.as_mut().unwrap().lock().unwrap().receive(ALU_CTRL_IN_ID, self.current_instruction[0..6].to_bitvec());
            self.sign_extend.as_mut().unwrap().lock().unwrap().receive(ALU_CTRL_IN_ID, self.current_instruction[0..16].to_bitvec());
            self.has_address = false;
        }
        
        
    }

    /// Set Functions
    pub fn set_control(&'a mut self, ctrl : Arc<Mutex<dyn Unit>>){
        self.control = Some(ctrl);
    }

    pub fn set_reg(&'a mut self, reg : Arc<Mutex<dyn Unit>>){
        self.reg = Some(reg);
    }

    pub fn set_signextend(&'a mut self, sign_extend: Arc<Mutex<dyn Unit>>){
        self.sign_extend = Some(sign_extend);
    } 

    pub fn set_aluctrl(&'a mut self, alu_ctrl: Arc<Mutex<dyn Unit>>){
        self.alu_ctrl = Some(alu_ctrl);
    }

    pub fn set_concater(&'a mut self, concater: Arc<Mutex<dyn Unit>>){
        self.concater = Some(concater);

    }
}

impl Unit for InstructionMemory{

    fn receive(&mut self, input_id: u32, address : Word){
        if input_id ==  IM_READ_ADDRESS_ID{
            self.current_address = address.to_bitvec().into_vec()[0];
            self.has_address = true;
        } else {
            //Message came on undefined input
        }
    }

    fn receive_signal(&mut self, _signal_id: u32, signal: bool) {
        // DO NOTHING
    }


}
