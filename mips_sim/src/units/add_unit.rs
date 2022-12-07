use bitvec::prelude::*;

mod add_unit {

    struct add_unit {

        addr : BitVec::<u32, LocalBits> ;
        sign_ext_instr : BitVec::<u32, LocalBits> ;
        has_instr: mut bool;
        has_addr : mut bool;
   
   
        mux : &impl Mux;


    }


    impl add_unit{
        //Define MUX id's
        pub fn new() -> instruction_memory{
            add_unit{

            }
        }

        ///Execute unit with thread
        pub fn execute(&self){

            if has_addr && has_instr{
                addr.
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

        fn add(word1 : BitVec::<u32, LocalBits> ,word2 : BitVec::<u32, LocalBits> ;){
                
        }


    }

    impl Unit for add_unit{

        pub fn receive(&self, input_id: u32, data : BitVec::<u32, LocalBits> ){
            if input_id == ADD_IN_1_ID{
                addr = data;
                has_addr;
            }else if input_id == ADD_IN_2_ID{
                sign_ext_instr = data;
                has_instr;
            }
        }

    }



}

