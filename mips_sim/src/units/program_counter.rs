use bitvec::prelude::*;

mod program_counter {

    struct program_counter {
        has_address: bool;
        current_address : Word;
        instruction_memory : &impl Unit;
        concater : &impl Unit;
        add_unit : &impl Unit;
        mux_branch: &impl (Unit + Mux);
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
                //Send address to instruction memory
                self.instruction_memory.receive(IM_READ_ADDRESS_ID, self.current_address.to_bitvec());

                //add 4 to address
                let added_address = add_4(self.current_address);

                //Send added address to concater
                self.concater.receive(CONC_IN_2_ID, added_address[28..32].to_bitvec());

                //Send added address to add-unit
                self.add_unit.receive(ADD_IN_1_ID,added_address.to_bitvec());

                //Send added address to mux-branch
                self.add_unit.receive(MUX_IN_0_ID, added_address.to_bitvec());

                self.has_address = false;
            }

        }


        fn add_4(addr : Word) -> Word {
            let num = addr.into_vec()[0];
            let res = num + 4;

            res.view_bits::<Lsb0>().to_bitvec();
        }
    }

    impl Unit for program_counter{

        pub fn receive(&self, input_id: u32, address : Word){
            if input_id == PC_IN_ID{
                self.current_address = address;
                self.has_address = true;
            }else{
                //Message came on undefined input
            }
            
        }
        

        pub fn set_instr_memory(&self, instr_mem: &impl Unit) {
            self.instr_mem = instr_mem;
        }

        pub fn set_concater(&self, concater: &impl Unit) {
            self.concater = concater;
        }

        pub fn set_add(&self, add: &impl Unit) {
            self.add_unit = add;
        }

        pub fn set_mux_branch(&self, mux: &impl (Unit+Mux)) {
            self.mux_branch = mux;
        }


    }

}