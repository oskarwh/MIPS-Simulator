use crate::units::unit::*;
use bitvec::prelude::*;
use crate::units::mux::*;
use std::sync::Mutex;

pub struct ProgramCounter<'a> {
    has_address: bool,
    current_address : Word,
    instruction_memory : Option<&'a Mutex<&'a mut Box<dyn  Unit>>>,
    concater : Option<&'a Mutex<&'a mut Box<dyn  Unit>>>,
    add_unit : Option<&'a Mutex<&'a mut Box<dyn  Unit>>>,
    mux_branch: Option<&'a Mutex<&'a mut Box<dyn  Unit>>>,
}


impl<'a>  ProgramCounter<'a>{
    pub fn new() -> ProgramCounter<'static>{
        ProgramCounter{
            current_address : bitvec![u32, Lsb0; 0; 32],
            has_address : true,

            instruction_memory: None,
            concater: None,
            add_unit: None,
            mux_branch: None,
        }
    }

    pub fn execute(&mut self){
        if self.has_address {
            //Send address to instruction memory
            self.instruction_memory.as_mut().unwrap().lock().unwrap().receive(IM_READ_ADDRESS_ID, self.current_address.to_bitvec());

            //add 4 to address
            let added_address = Self::add_4(self.current_address.to_bitvec());

            //Send added address to concater
            self.concater.as_mut().unwrap().lock().unwrap().receive(CONC_IN_2_ID, added_address[28..32].to_bitvec());

            //Send added address to add-unit
            self.add_unit.as_mut().unwrap().lock().unwrap().receive(ADD_IN_1_ID,added_address.to_bitvec());

            //Send added address to mux-branch
            self.mux_branch.as_mut().unwrap().lock().unwrap().receive(MUX_IN_0_ID, added_address.to_bitvec());

            self.has_address = false;
        }

    }


    fn add_4(addr : Word) -> Word {
        let num = addr.into_vec()[0];
        let res = num + 4;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    pub fn set_instr_memory(&'a mut self, instr_mem: &'a Mutex<&'a mut Box<dyn  Unit>>) {
        self.instruction_memory = Some(instr_mem);
    }

    pub fn set_concater(&'a mut self, concater: &'a Mutex<&'a mut Box<dyn  Unit>>) {
        self.concater = Some(concater);
    }

    pub fn set_add(&'a mut self, add: &'a Mutex<&'a mut Box<dyn  Unit>>) {
        self.add_unit = Some(add);
        todo!("Check if lifetime can be fixed?");
    }

    pub fn set_mux_branch(&'a mut self, mux: &'a Mutex<&'a mut Box<dyn  Unit>>) {
        self.mux_branch = Some(mux);
    }
}

impl Unit for ProgramCounter<'_>{
    fn receive(&mut self, input_id: u32, address : Word){
        if input_id == PC_IN_ID{
            self.current_address = address;
            self.has_address = true;
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        todo!()
    }
}

