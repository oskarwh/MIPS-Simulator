use bitvec::prelude::*;

mod instruction_memory {

    struct instruction_memory {

        instructions : Vec<BitArr::<LocalBits, usize> >;
        current_instruction : BitArr::<LocalBits, usize>;
        current_address : u32;
        has_address : mut bool;
   
        mux1 : &impl Mux;

        reg : &impl Unit;
        pc : &impl Unit;
        sign_extend : &impl Unit;
        alu_ctrl : &impl Unit;
        control : &impl Unit;

        //input id´s
        read_address_id: u32; 
    }


    impl instruction_memory{
        //Define input nad output-id´s. TODO: Think about how this can be done nicer
        const READ_ADDRESS = 1;
        const MUX1_ID = 1;
        const REG_READ_1_ID = 0;
        const REG_READ_2_ID = 0;
        const CTRL_IN_ID = 0;
        //      .
        //      .
        //TODO: extend this

        pub fn new(
            instr: Vec<u32>,
            mux1: &impl Mux, 
            reg: &impl Unit,
            pc: &impl Unit,
            sign_extend : &impl Unit,
            alu_ctrl : &impl Unit,
            ctrl : &impl Unit;

        ) -> instruction_memory{
            
            instruction_memory{
                instructions: instr, 
                has_address:false,
                mux1: mux1,
                mux2: mux2,
                reg:reg,
                pc:pc,
                sign_extend:sign_extend,
                alu_ctrl:alu_ctrl,
                control:ctrl,

                read_address_id : READ_ADDRESS,
            }
        }

        pub fn execute(&self){
            while(stop){

                if(has_address){
                    //Received address on read_address! Find corresponding instruction
                    current_instruction = instructions[current_address >> 2];

                    //Send instruction to other units
                    mux1.receive(MUX1_ID, current_instruction[0...25]<<2 );//Todo: Think about how to concat with ADD output
                    reg.receive(REG_READ_1_ID, current_instruction[21...25]);
                    reg.receive(REG_READ_2_ID, current_instruction[16...20]);
                    control.receive(CTRL_IN_ID, current_instruction[26...31]);
                }
                
            } 
        }

        pub fn stop(&self){
            stop =true;
        }

    }

    impl Unit for instruction_memory{


        pub fn receive(&self, input_id: u32, address : BitArr::<LocalBits, usize>){
            if input_id ==  self.read_address_id{
                current_address = address;
                has_address = true;
            }else{
                //Message came on undefined input
            }
            
        }
    }

}


