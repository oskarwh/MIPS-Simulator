use bitvec::prelude::*;

mod instruction_memory {

    struct instruction_memory {

        instructions : Vec<Word>;
        current_instruction : Word;
        current_address : u32;
        has_address : mut bool;

        reg : &impl Unit;
        pc : &impl Unit;
        sign_extend : &impl Unit;
        alu_ctrl : &impl Unit;
        control : &impl Unit;

    }


    impl instruction_memory{
        //Define MUX id's
        pub fn new(instr: Vec<Word>) -> instruction_memory{
            instruction_memory{instructions:instr}
        }


        ///Execute unit with thread
        pub fn execute(&self){
            while(stop){

                if(has_address){
                    //Received address on read_address! Find corresponding instruction. Need to right shift 2 steps (divide by 4)
                    current_instruction = instructions[current_address.shift_right(2).into_vec()[0]];

                    //Send instruction to other units
                    let borrow = &mut current_instruction[0...25];
                    borrow.shift_right(2);
                    concater.receive(CONC_IN_1_ID, borrow.to_bitvec() );
                    reg.receive(REG_READ_1_ID, current_instruction[21...25].to_bitvec());
                    reg.receive(REG_READ_2_ID, current_instruction[16...20].to_bitvec());
                    control.receive(CTRL_IN_ID, current_instruction[26...31].to_bitvec());
                    has_address = false;
                }
                
            } 
        }

        /// Set Functions
        pub fn setPC(&self, pc: &impl Unit;){
            self.pc = pc;
        }

        pub fn setControl(&self, ctrl : &impl Unit;){
            self.control = ctrl;
        }

        pub fn setReg(&self, reg : &impl Unit;){
            self.reg = reg;
        }

        pub fn setSignextend(&self, sign_extend: &impl Unit;){
            self.sign_extend = sign_extend;
        }

        pub fn setAluctrl(&self, alu_ctrl: &impl Unit;){
            self.alu_ctrl = alu_ctrl;
        }

        pub fn setConcater(&self, concater: &impl Unit;){
            self.concater = concater;
        }


        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }

    }

    impl Unit for instruction_memory{

        pub fn receive(&self, input_id: u32, address : Word){
            if input_id ==  IM_READ_ADDRESS_ID{
                current_address = address;
                has_address = true;
            }else{
                //Message came on undefined input
            }
            
        }
        
    }

}


