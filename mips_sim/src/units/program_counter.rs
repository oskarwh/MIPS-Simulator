use crate::units::unit::*;
use bitvec::prelude::*;
use crate::units::mux::*;

pub struct ProgramCounter<'a> {
    has_address: bool,
    current_address : Word,
    instruction_memory : &'a dyn Unit,
    concater : &'a dyn Unit,
    add_unit : &'a dyn  Unit,
    mux_branch: &'a dyn  Unit,
}


impl  ProgramCounter<'_>{
    
    pub fn new() -> ProgramCounter<'static>{

        ProgramCounter{
            current_address : bitvec![u32, Lsb0; 0; 32],
            has_address : true,

            instruction_memory: &EmptyUnit{},
            concater: &EmptyUnit{},
            add_unit: &EmptyUnit{},
            mux_branch: &EmptyUnit{},
        }
    }

    pub fn execute(&self){
        if self.has_address {
            //Send address to instruction memory
            self.instruction_memory.receive(IM_READ_ADDRESS_ID, self.current_address.to_bitvec());

            //add 4 to address
            let added_address = Self::add_4(self.current_address);

            //Send added address to concater
            self.concater.receive(CONC_IN_2_ID, added_address[28..32].to_bitvec());

            //Send added address to add-unit
            self.add_unit.receive(ADD_IN_1_ID,added_address.to_bitvec());

            //Send added address to mux-branch
            self.add_unit.receive(MUX_IN_0_ID, added_address.to_bitvec());

            self.has_address = false;
        }

    }


    fn add_4(addr : Word) -> Word {
        let num = addr.into_vec()[0];
        let res = num + 4;

        res.view_bits::<Lsb0>().to_bitvec()
    }


    

    pub fn set_instr_memory(&self, instr_mem: &impl Unit) {
        self.instruction_memory = instr_mem;
    }

    pub fn set_concater(&self, concater: &impl Unit) {
        self.concater = concater;
    }

    pub fn set_add(&self, add: &impl Unit) {
        self.add_unit = add;
    }

    pub fn set_mux_branch(&self, mux: &impl (Unit)) {
        self.mux_branch = mux;
    }
}

impl Unit for ProgramCounter<'_>{
    fn receive(&self, input_id: u32, address : Word){
        if input_id == PC_IN_ID{
            self.current_address = address;
            self.has_address = true;
        }else{
            //Message came on undefined input
        }
        
    }

    fn receive_signal(&self ,signal_id:u32) {
        todo!()
    }


}
