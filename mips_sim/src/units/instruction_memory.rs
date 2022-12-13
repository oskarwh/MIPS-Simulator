use bitvec::prelude::*;
use crate::units::unit::*;



pub struct InstructionMemory<'a> {

    instructions : Vec<Word>,
    current_instruction : Word,
    current_address : Word,
    has_address: bool,

    reg : Option<&'a dyn Unit>,
    sign_extend : Option<&'a dyn Unit>,
    alu_ctrl : Option<&'a dyn Unit>,
    control : Option<&'a dyn Unit>,
    concater: Option<&'a dyn Unit>,

}


impl InstructionMemory<'_>{

    pub fn new(instr: Vec<Word>) -> InstructionMemory<'static>{
        InstructionMemory{
            instructions:instr,
            current_instruction: bitvec![u32, Lsb0; 0; 32],
            current_address: bitvec![u32, Lsb0; 0; 32],
            has_address: false,

            reg: None,
            sign_extend: None,
            alu_ctrl: None,
            control: None,
            concater: None
        }

    }

        
    ///Execute unit with thread
    pub fn execute(&mut self){
      
        if self.has_address {
            //Received address on read_address! Find corresponding instruction. Need to right shift 2 steps (divide by 4)
            let borrow = self.current_address;
            borrow.shift_right(2);
            self.current_instruction = self.instructions[borrow.into_vec()[0] as usize];


            //Send to concater, word will be shifted left (shift_right because of the way BitVec is designed)
            let borrow = &mut self.current_instruction[0..26];
            borrow.shift_right(2);
            self.concater.unwrap().receive(CONC_IN_1_ID, borrow.to_bitvec() );

            //Send instruction to other units
            self.reg.unwrap().receive(REG_READ_1_ID, self.current_instruction[21..26].to_bitvec());
            self.reg.unwrap().receive(REG_READ_2_ID, self.current_instruction[16..21].to_bitvec());
            self.control.unwrap().receive(CTRL_IN_ID, self.current_instruction[26..32].to_bitvec());
            self.has_address = false;
        }
        
        
    }

    /// Set Functions

    pub fn set_control(&mut self, ctrl : &impl Unit){
        self.control = Some(ctrl);
    }

    pub fn set_reg(&mut self, reg : &impl Unit){
        self.reg = Some(reg);
    }

    pub fn set_signextend(&mut self, sign_extend: &impl Unit){
        self.sign_extend = Some(sign_extend);
    } 

    pub fn set_aluctrl(&mut self, alu_ctrl: &impl Unit){
        self.alu_ctrl = Some(alu_ctrl);
    }

    pub fn set_concater(&mut self, concater: &impl Unit){
        self.concater = Some(concater);
    }


}

impl Unit for InstructionMemory<'_>{

    fn receive(&mut self, input_id: u32, address : Word){
        if input_id ==  IM_READ_ADDRESS_ID{
            self.current_address = address;
            self.has_address = true;
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&mut self ,signal_id:u32) {
        // DO NOTHING
    }
    
}




