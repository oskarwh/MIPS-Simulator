use bitvec::prelude::*;

mod add_unit {

    struct add_unit {

        addr : Word;
        sign_ext_instr : Word;
        has_instr: mut bool;
        has_addr : mut bool;
   
   
        mux_branch : &impl (Unit+Mux);


    }


    impl add_unit{
        //Define MUX id's
        pub fn new() -> instruction_memory{
            add_unit{
                has_instr:false,
                has_address:false,
            }
        }

        ///Execute unit with thread
        pub fn execute(&self){

            if self.has_addr && self.has_instr{
                let res = add(self.addr, self.sign_ext_instr)
                self.mux_branch.receive(MUX_IN_1_ID, res);
            }
        }

        /// Set Functions
        pub fn set_mux_branch(&self, mux: &impl (Unit + Mux)){
            self.mux_branch = mux;
        }

        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }

        fn add(word1 : Word, word2 : Word) -> Word {
            let num1 = word1.into_vec()[0];
            let num2 = word12.into_vec()[0];

            let res = num1 + num1;

            res.view_bits::<Lsb0>().to_bitvec();
        }

    }

    impl Unit for add_unit{

        pub fn receive(&self, input_id: u32, data : Word){
            if input_id == ADD_IN_1_ID{
                self.addr = data;
                self.has_addr = true;
            }else if input_id == ADD_IN_2_ID{
                self.sign_ext_instr = data;
                self.has_instr = true;
            }
        }

    }



}

