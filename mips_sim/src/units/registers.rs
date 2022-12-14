use bitvec::prelude::*;
use crate::units::unit::*;



pub struct Registers<'a> {

    registers: Vec<Word> ,

    read1_reg : u32,
    read2_reg : u32,
    write_reg : u32,
    write_data : Word,

    has_read1 : bool,
    has_read2 : bool,
    has_write_reg : bool,
    has_write_data : bool,


    alu : Option<&'a mut dyn Unit>,
    mux_alu_src : Option<&'a mut dyn  Unit>,
    data_memory : Option<&'a mut dyn Unit>,

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

            read1_reg: 0,
            read2_reg: 0,
            write_reg: 0,
            write_data: bitvec![u32, Lsb0; 0; 32],

            alu: None,
            mux_alu_src: None,
            data_memory: None,
        }
    }


    ///Execute unit with thread
    pub fn execute(&mut self){
        if self.has_read1{
            //Received reg1! Find corresponding data and send to ALU
            let data = self.registers[self.read1_reg as usize].to_bitvec();
            self.alu.as_mut().unwrap().receive(ALU_IN_1_ID, data.to_bitvec());
            self.has_read1 = false;
        }

        if self.has_read2{
            //Received reg1! Find corresponding data and send to ALU-src mux and Data Memory
            let data = self.registers[self.read2_reg as usize].to_bitvec();
            self.mux_alu_src.as_mut().unwrap().receive(MUX_IN_0_ID, data.to_bitvec());
            self.data_memory.as_mut().unwrap().receive(DM_DATA_ID, data.to_bitvec());
            self.has_read2 = false;
        }

        if self.has_write_reg && self.has_write_data && self.reg_write_signal{
            //Got data to write and is signaled to write! Insert into registers on index write_reg
            self.registers[self.write_reg as usize] = self.write_data.to_bitvec();
        }
        
    
    }

    /// Set Functions
    pub fn set_alu(&mut self, alu: &mut dyn Unit){
        self.alu = Some(unsafe { std::mem::transmute(alu) });
    }

    pub fn set_mux_alu_src(&mut self, mux: &mut dyn Unit){
        self.mux_alu_src = Some(unsafe { std::mem::transmute(mux) });
    }

    pub fn set_data_memory(&mut self, data_memory: &mut dyn Unit){
        self.data_memory = Some(unsafe { std::mem::transmute(data_memory) });
    }


}

impl Unit for Registers<'_>{

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
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&mut self ,signal_id:u32){
        if signal_id == DEFAULT_SIGNAL{
            self.reg_write_signal = true;
        }
    }
    
}




