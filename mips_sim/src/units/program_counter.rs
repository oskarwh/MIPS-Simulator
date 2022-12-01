use bitvec::prelude::*;

mod program_counter {

    struct program_counter {

        instructions : Vec<BitArr::<LocalBits, usize> >;
        current_address : u32;
        pc : program_counter;
    }


    impl program_counter{
        pub fn new() -> program_counter{

            instruction_memory{instructions: instr, pc: pc}
        }

        pub fn execute(){
            while(){
                
            }
            
        }

        pub fn get_instruction(index_a: usize, index_b : usize)-> BitSlice::<LocalBits, usize>;{
            &instructions[current_address >> 2][index_a...index_b]
        }

        fn read_address(){
            current_address = pc.get_address();
        }

        

    }

}