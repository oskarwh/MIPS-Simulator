use bitvec::prelude::*;

mod instruction_memory {

    struct instruction_memory {

        instructions : Vec<BitVec::<LocalBits, usize> >;
        current_instruction : BitVec::<LocalBits, usize>;
        pc : program_counter;
    }


    impl instruction_memory{
        pub fn new(instr: Vec<u32>, pc: program_counter) -> instruction_memory{

            instruction_memory{instructions: instr, pc: pc}
        }

        pub fn execute(){

        }

        fn get_instruction()-> u32{
            instructions[address >> 2]
        }

        fn read_address(){
            current_instruction = pc.get_instruction();
        }

        

    }




}