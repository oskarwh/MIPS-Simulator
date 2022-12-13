use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;



pub struct Registers<'a> {

    registers: Vec<Word> ,

    read1_reg : Word,
    read2_reg : Word,
    write_reg : Word,
    write_data : Word,

    has_read1 : bool,
    has_read2 : bool,
    has_write_reg : bool,
    has_write_data : bool,


    alu : &'a dyn Unit,
    mux_alu_src : &'a dyn  Unit,
    data_memory : &'a dyn Unit,

    reg_write_signal : bool,

}


impl Registers<'_>{

    pub fn new() -> Registers<'static>{
        //Make registers and insert 0 into all of them
        const n_regs:usize = 32;
        let mut registers: Vec<Word> = vec![BitVec::new(); n_regs];

        //Create registers object
        Registers{
            registers,
            
            has_read1:false,
            has_read2:false,
            has_write_reg:false,
            has_write_data:false,
            reg_write_signal:false,

            read1_reg: bitvec![u32, Lsb0; 0; 32],
            read2_reg: bitvec![u32, Lsb0; 0; 32],
            write_reg: bitvec![u32, Lsb0; 0; 32],
            write_data: bitvec![u32, Lsb0; 0; 32],

            alu: &EmptyUnit{},
            mux_alu_src: &EmptyUnit{},
            data_memory: &EmptyUnit{},
        }
    }


    ///Execute unit with thread
    pub fn execute(&self){
        if self.has_read1{
            //Received reg1! Find corresponding data and send to ALU
            let data = self.registers[self.read1_reg.into_vec()[0] as usize];
            self.alu.receive(ALU_IN_1_ID, data.to_bitvec());
            self.has_read1 = false;
        }

        if self.has_read2{
            //Received reg1! Find corresponding data and send to ALU-src mux and Data Memory
            let data = self.registers[self.read2_reg.into_vec()[0] as usize];
            self.mux_alu_src.receive(MUX_IN_0_ID, data.to_bitvec());
            self.data_memory.receive(DM_DATA_ID, data.to_bitvec());
            self.has_read2 = false;
        }

        if self.has_write_reg && self.has_write_data && self.reg_write_signal{
            //Got data to write and is signaled to write! Insert into registers on index write_reg
            self.registers[self.write_reg.into_vec()[0] as usize] = self.write_data;
        }
        
    
    }

    /// Set Functions
    pub fn set_alu(&self, alu: &impl Unit){
        self.alu = alu;
    }

    pub fn set_mux_alu_src(&self, mux: &impl Unit){
        self.mux_alu_src = mux;
    }

    pub fn set_data_memory(&self, data_memory: &impl Unit){
        self.data_memory = data_memory;
    }


}

impl Unit for Registers<'_>{

    fn receive(&self, input_id: u32, data : Word){
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

    fn receive_signal(&self ,signal_id:u32){
        if signal_id == DEFAULT_SIGNAL{
            self.reg_write_signal = true;
        }
    }
    
}




