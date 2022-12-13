use crate::units::unit::*;
use bitvec::prelude::*;
use crate::units::mux::*;

pub struct ProgramCounter<'a> {
    has_address: bool,
    current_address : Word,
    instruction_memory : Option<&'a dyn Unit>,
    concater : Option<&'a dyn Unit>,
    add_unit : Option<&'a dyn  Unit>,
    mux_branch: Option<&'a dyn  Unit>,
}


impl  ProgramCounter<'_>{
    
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
            self.instruction_memory.unwrap().receive(IM_READ_ADDRESS_ID, self.current_address.to_bitvec());

            //add 4 to address
            let added_address = Self::add_4(self.current_address);

            //Send added address to concater
            self.concater.unwrap().receive(CONC_IN_2_ID, added_address[28..32].to_bitvec());

            //Send added address to add-unit
            self.add_unit.unwrap().receive(ADD_IN_1_ID,added_address.to_bitvec());

            //Send added address to mux-branch
            self.add_unit.unwrap().receive(MUX_IN_0_ID, added_address.to_bitvec());

            self.has_address = false;
        }

    }


    fn add_4(addr : Word) -> Word {
        let num = addr.into_vec()[0];
        let res = num + 4;

        res.view_bits::<Lsb0>().to_bitvec()
    }


    

    pub fn set_instr_memory(&mut self, instr_mem: &impl Unit) {
        self.instruction_memory = Some(instr_mem);
    }

    pub fn set_concater(&mut self, concater: &impl Unit) {
        self.concater = Some(concater);
    }

    pub fn set_add(&mut self, add: &impl Unit) {
        self.add_unit = Some(add);
    }

    pub fn set_mux_branch(&mut self, mux: &impl (Unit)) {
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

    fn receive_signal(&mut self ,signal_id:u32) {
        todo!()
    }


}
