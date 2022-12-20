use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Mutex;
use std::sync::Arc;


pub struct Registers {

    data_recently_written: bool,
    prev_register_index: usize,

    registers: Vec<Word>, 

    read1_reg : u32,
    read2_reg : u32,
    write_reg : u32,
    write_data : Word,

    has_read1 : bool,
    has_read2 : bool,
    has_write_reg : bool,
    has_write_data : bool,


    alu : Option<Arc<Mutex<dyn Unit>>>,
    mux_alu_src : Option<Arc<Mutex<dyn Unit>>>,
    mux_jr : Option<Arc<Mutex<dyn Unit>>>,
    data_memory : Option<Arc<Mutex<dyn Unit>>>,

    reg_write_signal : bool,
}


impl Registers {

    pub fn new() -> Registers{
        //Make registers and insert 0 into all of them
        const N_REGS:usize = 32;
        let mut registers: Vec<Word> = vec![bitvec![u32, Lsb0; 0; 32]; N_REGS];

        //Create registers object
        Registers{
            data_recently_written: false,
            prev_register_index: 0,

            registers,
            
            has_read1:false,
            has_read2:false,
            has_write_reg:false,
            has_write_data:false,
            reg_write_signal:false,

            read1_reg: 0,
            read2_reg: 0,
            write_reg: 0,
            write_data: bitvec![u32, Lsb0; 0; 32],

            alu: None,
            mux_alu_src: None,
            data_memory: None,
            mux_jr: None,
        }
    }




    /// Set Functions
    pub fn set_alu(& mut self, alu: Arc<Mutex<dyn Unit>>){
        self.alu = Some(alu);
    }

    pub fn set_mux_alu_src(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_alu_src = Some(mux);
    }

    pub fn set_data_memory(&mut self, data_memory: Arc<Mutex<dyn Unit>>){
        self.data_memory = Some(data_memory);
    }

    pub fn set_mux_jr(&mut self, mux: Arc<Mutex<dyn Unit>>){
        self.mux_jr = Some(mux);
    }

    pub fn get_changed_register(&self) -> (u32, usize) {
        return (self.registers[self.prev_register_index].clone().into_vec()[0], self.prev_register_index);
    }

    pub fn instruction_completed(&mut self) -> bool {
        if self.data_recently_written {
            self.data_recently_written = false;
            true
        }else {
            false
        }    
    }

}

impl Unit for Registers {

    fn receive(&mut self, input_id: u32, data : Word){
        if input_id ==  REG_READ_1_ID{
            self.read1_reg = data.to_bitvec().into_vec()[0];
            self.has_read1 = true;
        }else if input_id ==  REG_READ_2_ID{
            self.read1_reg = data.to_bitvec().into_vec()[0];
            self.has_read2 = true;
        }else if input_id ==  REG_WRITE_REG_ID{
            self.write_reg = data.to_bitvec().into_vec()[0];
            self.has_write_reg = true;
        }else if input_id ==  REG_WRITE_DATA_ID{
            self.write_data = data;
            self.has_write_data = true;
            // Instruction about to be completed
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool){
        if signal_id == DEFAULT_SIGNAL{
            self.reg_write_signal = signal;
        }
    }

    ///Execute unit with thread
    fn execute(&mut self){
        if self.has_read1{
            //Received reg1! Find corresponding data and send to ALU
            let data = self.registers[self.read1_reg as usize].to_bitvec();

            self.alu.as_mut().unwrap().lock().unwrap().receive(ALU_IN_1_ID, data.to_bitvec());
            // Send to Jump Register mux
            self.mux_jr.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, data.to_bitvec());
            self.has_read1 = false;
        }

        if self.has_read2{
            //Received reg1! Find corresponding data and send to ALU-src mux and Data Memory
            let data = self.registers[self.read2_reg as usize].to_bitvec();
            self.mux_alu_src.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID, data.to_bitvec());
            self.data_memory.as_mut().unwrap().lock().unwrap().receive(DM_DATA_ID, data.to_bitvec());
            self.has_read2 = false;
        }

        if self.has_write_reg && self.has_write_data && self.reg_write_signal{
            // Reset register bool
            self.prev_register_index = self.write_reg as usize;

            //Got data to write and is signaled to write! Insert into registers on index write_reg
            self.registers[self.write_reg as usize] = self.write_data.to_bitvec();
            self.has_write_reg = false;
            self.has_write_data = false;
            // Data was written to register, which means instruction is done
            self.data_recently_written = true;
        }
        
    
    }
    
}




