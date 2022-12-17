use crate::units::unit::*;
use bitvec::prelude::*;
use crate::units::mux::*;

pub struct ProgramCounter<'a> {
    has_address: bool,

    current_address : Word,

    to_instruction_mem:Word,
    to_concater:Word,
    to_add:Word,
    to_mux_branch:Word,


    instruction_memory : Option<&'a mut dyn Unit>,
    concater : Option<&'a mut dyn Unit>,
    add_unit : Option<&'a mut dyn  Unit>,
    mux_branch: Option<&'a mut dyn  Unit>,
}


impl<'a>  ProgramCounter<'_>{
    
    pub fn new() -> ProgramCounter<'static>{

        ProgramCounter{
            current_address : bitvec![u32, Lsb0; 0; 32],
            has_address : true,

            instruction_memory: None,
            concater: None,
            add_unit: None,
            mux_branch: None,

            to_instruction_mem: bitvec![u32, Lsb0; 0; 32],
            to_concater: bitvec![u32, Lsb0; 0; 32],
            to_add: bitvec![u32, Lsb0; 0; 32],
            to_mux_branch: bitvec![u32, Lsb0; 0; 32],
        }
    }

    pub fn execute(&mut self){
        if self.has_address {
            //Send address to instruction memory
            self.to_mux_branch = self.current_address;
            self.instruction_memory.as_mut().unwrap().ping(IM_READ_ADDRESS_ID, self);

            //add 4 to address
            let added_address = Self::add_4(self.current_address.to_bitvec());
            
            //Send added address to concater
            self.to_concater = added_address[28..32].to_bitvec();
            self.concater.as_mut().unwrap().ping(CONC_IN_2_ID, self);

            //Send added address to add-unit
            self.to_add = added_address;
            self.add_unit.as_mut().unwrap().ping(ADD_IN_1_ID, self);

            //Send added address to mux-branch
            self.to_mux_branch = added_address;
            self.mux_branch.as_mut().unwrap().ping(MUX_IN_0_ID, self);

            self.has_address = false;
        }
    }


    fn add_4(addr : Word) -> Word {
        let num = addr.into_vec()[0];
        let res = num + 4;

        res.view_bits::<Lsb0>().to_bitvec()
    }

    pub fn set_instr_memory(&mut self, instr_mem: &mut dyn  Unit) {
        self.instruction_memory = Some(unsafe { std::mem::transmute(instr_mem) });
    }

    pub fn set_concater(&mut self, concater: &mut dyn  Unit) {
        self.concater = Some(unsafe { std::mem::transmute(concater) });
    }

    pub fn set_add(&mut self, add: &mut dyn  Unit) {
        self.add_unit = Some(unsafe { std::mem::transmute(add) });
    }

    pub fn set_mux_branch(&mut self, mux: &mut dyn  Unit) {
        self.mux_branch = Some(unsafe { std::mem::transmute(mux) });
    }
}

impl Unit for ProgramCounter<'_>{

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        todo!()
    }

    fn ping(&self, input_id : u32, source:&dyn Unit) {
        if input_id == PC_IN_ID{
            self.current_address = source.get_data(input_id);
            self.has_address = true;
        }else{
            //Message came on undefined input
        }
    }

    fn get_data(&self, input_id : u32)-> Word {
        match input_id{
            IM_READ_ADDRESS_ID=>
                return self.to_instruction_mem.to_bitvec(),
            CONC_IN_2_ID=>
                return self.to_concater.to_bitvec(),
            ADD_IN_1_ID=>
                return self.to_add.to_bitvec(),
            MUX_IN_0_ID=>
                return self.to_mux_branch.to_bitvec(),
        }
    }


}
