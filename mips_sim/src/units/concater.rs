use bitvec::prelude::*;

mod concater {

    struct concater {

        addr : BitVec::<u32, LocalBits> ;
        instr : BitVec::<u32, LocalBits> ;
        has_instr: mut bool;
        has_addr : mut bool;
   
        mux : &impl Mux;

    }


    impl concater{

        pub fn new() -> instruction_memory{
            concater{instructions:instr}
        }


        ///Execute unit with thread
        pub fn execute(&self){

            if has_addr && has_instr{
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

        pub fn receive(&self, input_id: u32, data : BitVec::<u32, LocalBits> ){
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


