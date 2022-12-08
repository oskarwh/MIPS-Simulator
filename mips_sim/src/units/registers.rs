use bitvec::prelude::*;

mod registers {

    struct registers {

        registers: Vec<Word>;

        read1_reg : Word;
        read2_reg : Word;
        write_reg : Word;
        write_data : Word;

        has_read1 : mut bool;
        has_read2 : mut bool;
        has_write_reg : mut bool;
        has_write_data : mut bool;


        alu : &impl Unit;
        mux_alu_src : &impl (Unit+Mux);
        data_memory : &impl Unit;

        reg_write_signal : mut bool;

    }


    impl registers{
    
        pub fn new() -> registers{
            //Make registers and insert 0 into all of them
            let const n_regs = 32;
            let mut registers: [Word; n_regs] = vec![BitVec::new(); n_regs].into_boxed_slice();

            //Create registers object
            registers{
                registers:registers,
                has_read1:false,
                has_read2:false,
                has_write:false,
                has_data:false,
                reg_write_signal:false,
            }
        }


        ///Execute unit with thread
        pub fn execute(&self){
            while(stop){

                if self.has_read1{
                    //Received reg1! Find corresponding data and send to ALU
                    data = registers[read1_reg.into_vec()[0]];
                    alu.receive(ALU_IN_1_ID, data.to_bitvec());
                    has_read1 = false;
                }

                if self.has_read2{
                    //Received reg1! Find corresponding data and send to ALU-src mux and Data Memory
                    let data = self.registers[self.read2_reg.into_vec()[0]];
                    self.mux_alu_src.receive(MUX_IN_0_ID, data.to_bitvec());
                    self.data_memory.receive(DM_DATA_ID, data.to_bitvec());
                    self.has_read2 = false;
                }

                if self.has_write_reg && self.has_write_data && self.reg_write_signal{
                    //Got data to write and is signaled to write! Insert into registers on index write_reg
                    self.registers[self.write_reg.into_vec()[0]] = self.write_data;
                }
                
            } 
        }

        /// Set Functions
        pub fn set_alu(&self, alu: &impl Unit){
            self.alu = alu;
        }

        pub fn set_mux_alu_src(&self, mux: &impl (Unit+Mux)){
            self.mux_alu_src = mux;
        }

        pub fn set_data_memory(&self, data_memory: &impl Unit){
            self.data_memory = data_memory;
        }
    

        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }

    }

    impl Unit for registers{

        pub fn receive(&self, input_id: u32, data : Word){
            if input_id ==  REG_READ_1_ID{
                self.read1_reg = data;
                self.has_read1 = true;
            }else if input_id ==  REG_READ_2_ID{
                self.read1_reg = data;
                self.has_read2 = true;
            }else if input_id ==  REG_WRITE_REG_ID{
                self.write_reg = data;
                self.has_write_reg = true;
            }else if input_id ==  REG_WRITE_DATA_ID{
                self.write_data = data;
                self.has_write_data = true;
            }else{
                //Message came on undefined input
            }
            
        }

        pub fn receive_signal(&self ,signal_id:u32){
            if signal_id == default_signal{
                self.reg_write_signal = true;
            }
        }
        
    }

}


