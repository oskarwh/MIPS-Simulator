use bitvec::prelude::*;

mod instruction_memory {

    struct instruction_memory {

        instructions : Vec<BitArr::<LocalBits, usize> >;
        current_instruction : BitArr::<LocalBits, usize>;
        current_address : u32;
        has_address : mut bool;
        pc : &program_counter;
        reg : &registers;
        sign_extend : &sign_extend;
        alu_ctrl : &alu_control;
    }


    impl instruction_memory{
        pub fn new(instr: Vec<u32>, pc: program_counter) -> instruction_memory{

            instruction_memory{instructions: instr, has_address:false}
        }

        pub fn execute(&self){
            while(stop){

                if(has_address){
                    current_address= pc.get_address();
                    current_instruction = instructions[current_address >> 2];
                    has_instruction = true;
                }
                
            } 
        }

        pub fn stop(){
            stop =true;
        }

        fn get_instruction(index_a: usize, index_b : usize)-> BitSlice::<LocalBits, usize>;{
            current_instruction[index_a...index_b]
        }

    }

 

}


/*  
    impl instruction_memory{
        pub fn new(instr: Vec<u32>, pc: program_counter) -> instruction_memory{

            instruction_memory{instructions: instr, has_address:false}
        }

        pub fn execute(&self){
            while(stop){

                if(pc.has_address()){
                    current_address= pc.get_address();
                    current_instruction = instructions[current_address >> 2];
                    has_instruction = true;
                }
                
            } 
        }

        pub fn stop(){
            stop =true;
        }

        fn get_instruction(index_a: usize, index_b : usize)-> BitSlice::<LocalBits, usize>;{
            current_instruction[index_a...index_b]
        }

    }

    impl instruction_memory for MuxInputer{
        fn get_mux_input(&self) -> BitArr::<LocalBits, usize>{

        }
    }

}*/