use bitvec::prelude::*;

mod concater {

    struct concater {

        addr : Word;
        instr : Word;
        has_instr: mut bool;
        has_addr : mut bool;
   
        mux_jump : &impl (Unit + Mux);

    }


    impl concater{

        pub fn new() -> instruction_memory{
            concater{
                has_addr:false,
                has_instr:false,
            }
        }


        ///Execute unit with thread
        pub fn execute(&self){

            if self.has_addr && self.has_instr{
                //Append bits from instruction memory with address from PC+4
                self.addr.append(&instr);
                self.mux_jump.receive(MUX_IN_1_ID, self.addr.to_bitvec());
            }
        }

        /// Set Functions
        pub fn set_mux_jump(&self, mux: &impl (Unit + Mux);){
            self.mux_jump = mux;
        }

        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }

    }

    impl Unit for concater{

        pub fn receive(&self, input_id: u32, data : Word){
            if input_id == CONC_IN_1_ID{
                self.instr = data;
                self.has_instr = true;
            }else if input_id == CONC_IN_2_ID{
                self.addr = data;
                self.has_addr = true;
            }
        }

        
    }

}


