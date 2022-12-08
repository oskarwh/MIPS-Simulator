use bitvec::prelude::*;

mod program_counter {

    struct program_counter {
        has_address: bool;
        current_address : Word;
        instruction_memory : &impl Unit;
    }


    impl program_counter{
        
        pub fn new() -> program_counter{
            let first_address = 0u32;
            let first_addr_word = first_address.view_bits::<Lsb0>();

            program_counter{
                current_address : first_addr_word,
                has_address : true,
            }
        }

        pub fn execute(){
            if has_address {
                instruction_memory.receive(IM_READ_ADDRESS_ID, current_address);
                has_address = false;
            }

        }
    }

    impl Unit for program_counter{

        pub fn receive(&self, input_id: u32, address : Word){
            if input_id == PC_IN_ID{
                current_address = address;
                has_address = true;
            }else{
                //Message came on undefined input
            }
            
        }
        

        pub fn set_instr_memory(&self, instr_mem: &Unit) {
            self.instr_mem = instr_mem;
        }

    }

}