use bitvec::prelude::*;
use crate::units::unit::*;

pub struct AluControl<'a> {
    alu_op0: bool,
    alu_op1: bool,

    alu_unit: Option<&'a mut dyn Unit>,
}

impl AluControl<'_> {
    pub fn new () -> AluControl<'static>{
        AluControl { 
            alu_op0: false,
            alu_op1: false,
            alu_unit: None, 
        }
    }

    pub fn execute() {
        
    }


    pub fn set_add_signals(&mut self) {
        // Send 0010 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
    }

    pub fn set_sub_signals(&mut self) {
        // Send 0110 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL1_SIGNAL);
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
    }

    pub fn set_and_signals(&mut self) {
        // Send 0000 to ALU
        // We set no signals
    }

    pub fn set_or_signals(&mut self) {
        // Send 0001 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL3_SIGNAL);
    }

    pub fn set_slt_signals(&mut self) {
        // Send 0111 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL1_SIGNAL);
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL3_SIGNAL);
    }

    pub fn set_lw_signals(&mut self) {
        // Send 0010 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
    }

    pub fn set_sw_signals(&mut self) {
        // Send 0010 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
    }

    pub fn set_bra_signals(&mut self) {
        // Send 0110 to ALU
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL1_SIGNAL);
        self.alu_unit.as_mut().unwrap().receive_signal(ALU_CTRL2_SIGNAL);
    }

    pub fn set_alu(&mut self, alu: &mut dyn Unit) {
        self.alu_unit = Some(unsafe { std::mem::transmute(alu) });
    }
}

impl Unit for AluControl<'_>{
    fn receive_signal(&mut self ,signal_id:u32) {
        if(signal_id == ALU_OP0_SIGNAL) {
            self.alu_op0 = true;
        }else if(signal_id == ALU_OP1_SIGNAL){
            self.alu_op1 = true;
        }
    }
    
    fn receive (&mut self, input_id : u32, data :BitVec::<u32, LocalBits>) {
        if(input_id == ALU_CTRL_IN_ID) {
            // Bit vector for a add instruction comming to the alu
            let add_bitvec =  bitvec![1,0,0,0,0,0];
            match data {
                add_bitvec => 
                    // Add command
                    self.set_add_signals(),
            }
        }else {
            // Wrong ID
        }
        
    }
}