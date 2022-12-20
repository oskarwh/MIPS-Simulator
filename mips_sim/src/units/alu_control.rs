use std::sync::{Mutex, Arc};

use bitvec::prelude::*;
use crate::units::unit::*;




pub struct AluControl{
    alu_op0: bool,
    alu_op1: bool,
    alu_op2: bool,
    funct: Word,

    alu_unit: Option<Arc<Mutex<dyn Unit>>>,

    has_op0: bool,
    has_op1: bool,
    has_op2: bool,
    has_funct: bool,
}


impl<'a> AluControl {
    pub fn new () -> AluControl{
        AluControl { 
            alu_op0: false,
            alu_op1: false,
            alu_op2: false,
            funct: bitvec![u32, Lsb0; 0; 6],
            alu_unit: None, 
            has_op0: false,
            has_op1: false,
            has_op2: false,
            has_funct: false,
        }
    }


    pub fn set_add_signals(&mut self) {
        // Send 0010 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
    }

    pub fn set_sub_signals(&mut self) {
        // Send 0110 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
    }

    pub fn set_and_signals(&mut self) {
        // Send 0000 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
    }

    pub fn set_or_signals(&mut self) {
        // Send 0001 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
    }

    pub fn set_slt_signals(&mut self) {
        // Send 0111 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
    }

    pub fn set_nor_signals(&mut self) {
        // Send 1100 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, true);
    }

    pub fn set_srl_signals(&mut self) {
        // Send 1101 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, true);
    }

    pub fn set_sra_signals(&mut self) {
        // Send 1011 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, true);
    }

    pub fn set_alu(&mut self, alu: Arc<Mutex<dyn Unit>>) {
        self.alu_unit = Some(alu);
    }

    fn reset_bools(&mut self) {
        self.has_funct = false;
        self.has_op0 = false;
        self.has_op1 = false;
        self.has_op2 = false;
    }
}

impl Unit for AluControl {
    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        if signal_id == ALU_OP0_SIGNAL {
            self.alu_op0 = signal;
            self.has_op0 = true;
        }else if signal_id == ALU_OP1_SIGNAL {
            self.alu_op1 = signal;
            self.has_op1 = true;
        }else if signal_id == ALU_OP2_SIGNAL {
            self.alu_op2 = signal;
            self.has_op2 = true;
        }
    }
    
    fn receive (&mut self, input_id : u32, data :BitVec::<u32, LocalBits>) {
        if input_id == ALU_CTRL_IN_ID {
            self.funct = data;
            self.has_funct = true;
        }else {
            // Wrong ID
        }
        
    }

    fn execute(&mut self) {
        if self.has_op0 && self.has_op1 && self.has_op2 && self.has_funct{
            // Check if instuction is r type
           if !self.has_op2 && self.alu_op1 && !self.alu_op0 {

           /* add_bitvec: bitvec![u32, Lsb0; 1,0,0,0,0,0],
            sub_bitvec: bitvec![u32, Lsb0; 1,0,0,0,1,0],
            and_bitvec: bitvec![u32, Lsb0; 1,0,0,1,0,0],
            or_bitvec: bitvec![u32, Lsb0; 1,0,0,1,0,1],
            slt_bitvec: bitvec![u32, Lsb0; 1,0,1,0,1,0],
            jr_bitvec: bitvec![u32, Lsb0; 0,0,1,0,0,0],
            nor_bitvec: bitvec![u32, Lsb0; 1,0,0,1,1,1],
            srl_bitvec: bitvec![u32, Lsb0; 0,0,0,0,1,0],
            sra_bitvec: bitvec![u32, Lsb0; 0,0,0,0,1,1],*/

                // Check which r type alu should do
                match self.funct.to_bitvec().into_vec()[0] {
                    // Add instruction
                    0b100000 =>
                        self.set_add_signals(),
                    
                    // Sub instruction
                    0b100010 =>
                        self.set_sub_signals(),

                    // And instruction
                    0b100100 =>
                        self.set_and_signals(),

                    // Or instruction
                    0b100101 =>
                        self.set_or_signals(),
                    
                    // Set On Less Than instruction
                    0b101010 =>
                        self.set_slt_signals(),

                    // Jr instruction
                    0b001000 =>
                        todo!(),
                        // What should i send to the alu?
                        // I do not need to du anything in the alu here i think?
                    
                    // Nor instruction
                    0b100111 =>
                        self.set_nor_signals(),

                    // Srl instruction
                    0b000010 =>
                        self.set_srl_signals(),
                    
                    // Sra instruction
                    0b000011 =>
                        self.set_sra_signals(),
                    //DO NOTHING
                    _ =>(),
                    //DO NOTHING
                }
            // Check for ori
            } else if self.has_op2 && !self.alu_op1 && !self.alu_op0 {
                self.set_or_signals();

            // Check for addi
            } else if !self.has_op2 && !self.alu_op1 && !self.alu_op0 {
                 self.set_add_signals();
        
            // Check for branch
            }else if !self.has_op2 && !self.alu_op1 && self.alu_op0 {
                self.set_sub_signals();
            }

        }
        // Make object ready to recieve new data
        self.reset_bools();
    }

}
