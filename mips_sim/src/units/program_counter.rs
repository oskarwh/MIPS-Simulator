use bitvec::prelude::*;

mod program_counter {

    struct program_counter {

        current_address : BitVec::<LocalBits, usize>;
        instr_mem: instruction_memory;
    }


    impl Unit for program_counter{
        pub fn new() -> program_counter{
            //instruction_memory{instructions: instr, pc: pc}
            current_address = bitvec![0;32];
        }

        pub fn execute(){
            while(){
                // Check when to send new instruction to instruction memory

                // Send next address to instr memmory
                instr_mem.receive_signal(self.IM_READ_ADDRESS_ID, current_address)
            }
        }

        pub fn receive(&self, input_id : u32, data : BitVec::<LocalBits, usize>) {
            // Set current address to the incoming adress.
            current_address = data;
        }

        pub fn set_instr_memory(&self, instr_mem: &Unit) {
            self.instr_mem = instr_mem;
        }
    }

}