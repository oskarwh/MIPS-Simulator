use bitvec::prelude::*;

mod concater {

    struct concater {

        addr : Word;
        instr : Word;
        has_instr: mut bool;
        has_addr : mut bool;
   
        mux : &impl Mux;

    }


    impl concater{

        pub fn new() -> instruction_memory{
            concater{}
        }


        ///Execute unit with thread
        pub fn execute(&self){

            if has_addr && has_instr{
                //Append bits from instruction memory with address from PC+4
                addr.append(&instr);
                mux.receive(MUX_IN_1_ID, addr.to_bitvec());
            }
        }

        /// Set Functions
        pub fn setMux(&self, mux: &impl (Unit + Mux);){
            self.mux = mux;
        }

        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }

    }

    impl Unit for concater{

        pub fn receive(&self, input_id: u32, data : Word){
            if input_id == CONC_IN_1_ID{
                instr = data;
                has_instr;
            }else if input_id == CONC_IN_2_ID{
                addr = data;
                has_addr;
            }
        }

        
    }

}


