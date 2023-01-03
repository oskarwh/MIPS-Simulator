use std::sync::{Mutex, Arc};

use bitvec::prelude::*;
use crate::units::unit::*;

/// A MIPS simulator unit. A controller for the ALU, will give the ALU
/// different instruction depending on signals from the Controll and 
/// the different function codes for the R-type instruction.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// AluControl Struct
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

/// AluControl Implementation
impl<'a> AluControl {
    /// Returns a new AluControl.
    ///
    /// # Returns
    ///
    /// * AluControl
    ///
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

    /// Sets ALU Control signals to inform ALU to perform an addition on incoming data.
    pub fn set_add_signals(&mut self) {
        // Send 0010 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an subtraction on incoming data.
    pub fn set_sub_signals(&mut self) {
        // Send 0110 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an Bitwise And on incoming data.
    pub fn set_and_signals(&mut self) {
        // Send 0000 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an Bitwise Or on incoming data.
    pub fn set_or_signals(&mut self) {
        // Send 0001 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an Set on Less Than on incoming data.
    pub fn set_slt_signals(&mut self) {
        // Send 0111 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, true);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an Bitwise Nor on incoming data.
    pub fn set_nor_signals(&mut self) {
        // Send 1100 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, true);
        lock.receive_signal(ALU_CTRL3_SIGNAL, true);
        lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Sets ALU Control signals to inform ALU to perform an Shiftleft Logical on incoming data.
    pub fn set_srl_signals(&mut self) {
        // Send 1101 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, false);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, true);
    }

    /// Sets ALU Control signals to inform ALU to perform an Shiftleft Arithmetic on incoming data.
    pub fn set_sra_signals(&mut self) {
        // Send 1011 to ALU
        let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
        lock.receive_signal(ALU_CTRL0_SIGNAL, true);
        lock.receive_signal(ALU_CTRL1_SIGNAL, false);
        lock.receive_signal(ALU_CTRL2_SIGNAL, false);
        lock.receive_signal(ALU_CTRL3_SIGNAL, false);
        lock.receive_signal(ALU_CTRL4_SIGNAL, true);
    }

    /// Set ALU Control signals to a "Default State", only used when ALU is not needed.
    /// However ALU need to send data for "data chain" to be completed.
    pub fn set_signals_false(&mut self) {
         // Send 00000 to ALU
         let mut lock = self.alu_unit.as_mut().unwrap().lock().unwrap();
         lock.receive_signal(ALU_CTRL0_SIGNAL, false);
         lock.receive_signal(ALU_CTRL1_SIGNAL, false);
         lock.receive_signal(ALU_CTRL2_SIGNAL, false);
         lock.receive_signal(ALU_CTRL3_SIGNAL, false);
         lock.receive_signal(ALU_CTRL4_SIGNAL, false);
    }

    /// Set a ALU that the 'AluControl' which is called on, should send signals to.
    /// 
    /// # Arguments
    ///
    /// * `alu` - The ALU that should be set
    ///
    pub fn set_alu(&mut self, alu: Arc<Mutex<dyn Unit>>) {
        self.alu_unit = Some(alu);
    }

    /// Resets bools that holds wether or not incoming signals and function code has been recived.
    fn reset_bools(&mut self) {
        self.has_funct = false;
        self.has_op0 = false;
        self.has_op1 = false;
        self.has_op2 = false;
    }
}

/// AluControl implementing Unit trait.
impl Unit for AluControl {

    /// Receives signal from a Control, comes with ID to 
    /// specify which signal.
    /// 
    /// # Arguments
    /// 
    /// * `signal_id` - Id to know what type of signal is comming
    /// * `signal` - Bool which holds state of signal (high/low)
    /// 
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
    
    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive (&mut self, input_id : u32, data : Word) {
        if input_id == ALU_CTRL_IN_ID {
            self.funct = data;
            self.has_funct = true;
        }else {
            // Wrong ID
        }
        
    }

    /// Checks if all data and signals needed has been received.
    /// If that is the case the ALU Control will check which signals 
    /// to send to the ALU.
    fn execute(&mut self) {
        if self.has_op0 && self.has_op1 && self.has_op2 && self.has_funct{
            // Check if instuction is r type
           if !self.alu_op2 && self.alu_op1 && !self.alu_op0 {

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
                        self.set_signals_false(),
                    
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
                }
            // Check for ori
            } else if self.alu_op2 && !self.alu_op1 && !self.alu_op0 {
                self.set_or_signals();

            // Check for addi
            } else if !self.alu_op2 && !self.alu_op1 && !self.alu_op0 {
                 self.set_add_signals();
        
            // Check for branch
            }else if !self.alu_op2 && !self.alu_op1 && self.alu_op0 {
                self.set_sub_signals();
            }

            // Make object ready to recieve new data
            self.reset_bools();
        }

    }

}
