use bitvec::prelude::*;

mod instruction_memory {

    struct InstructionMemory {

        instructions : Vec<Word>,
        current_instruction : Word,
        current_address : u32,
        has_address: bool,
   
        reg : &impl Unit,
        pc : &impl Unit,
        sign_extend : &impl Unit,
        alu_ctrl : &impl Unit,
        control : &impl Unit,

    }


    impl InstructionMemory{

        fn new(instr: Vec<Word>) -> InstructionMemory{
            InstructionMemory{
                instructions:instr,
                current_instruction: bitvec![u32, Lsb0; 0; 32],
                current_address: bitvec![u32, Lsb0; 0; 32],
                has_address: false,

                reg: Default::default(),
                pc: Default::default(),
                sign_extend: Default::default(),
                alu_ctrl: Default::default(),
                control: Default::default(),
            }

        }

            
        ///Execute unit with thread
        fn execute(&self){
            while(stop){

                if(self.has_address){
                    //Received address on read_address! Find corresponding instruction. Need to right shift 2 steps (divide by 4)
                    let borrow = self.current_address.shift_right(2);
                    self.current_instruction = self.instructions[borrow.into_vec()[0]];


                    //Send to concater, word will be shifted left (shift_right because of the way BitVec is designed)
                    let borrow = &mut self.current_instruction[0..26];
                    borrow.shift_right(2);
                    self.concater.receive(CONC_IN_1_ID, borrow.to_bitvec() );

                    //Send instruction to other units
                    self.reg.receive(REG_READ_1_ID, &self.current_instruction[21..26].to_bitvec());
                    self.reg.receive(REG_READ_2_ID, &self.current_instruction[16..21].to_bitvec());
                    self.control.receive(CTRL_IN_ID, &self.current_instruction[26..32].to_bitvec());
                    self.has_address = false;
                }
                
            }
        }

        /// Set Functions
        fn set_pc(&self, pc: &impl Unit){
            self.pc = pc;
        }

        fn set_control(&self, ctrl : &impl Unit){
            self.control = ctrl;
        }

        fn set_reg(&self, reg : &impl Unit){
            self.reg = reg;
                }

        fn set_signextend(&self, sign_extend: &impl Unit){
            self.sign_extend = sign_extend;
            } 

        fn set_aluctrl(&self, alu_ctrl: &impl Unit){
            self.alu_ctrl = alu_ctrl;
        }

        fn set_concater(&self, concater: &impl Unit){
            self.concater = concater;
        }


        ///Stop this thread
        fn stop(&self){
            stop =true;
        }

    }

    impl Unit for InstructionMemory{

        fn receive(&self, input_id: u32, address : Word){
            if input_id ==  IM_READ_ADDRESS_ID{
                self.current_address = address;
                self.has_address = true;
            }else{
                //Message came on undefined input
            }
            
        }
        
    }

}


