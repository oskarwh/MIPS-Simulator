use crate::units::unit::*;
use std::sync::{Mutex, Arc};

/// A MIPS simulator unit. Sets High or Low signals to different Units
/// in the simulator based on the incoming Instruction Word from Instruction Memory.
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-12-28: First complete version.

/// Control Struct
pub struct Control {
    mux_reg_dst: Arc<Mutex<dyn Unit>>,
    mux_jump: Arc<Mutex<dyn Unit>>,
    ander_branch:Arc<Mutex<dyn Unit>>,
    mux_alu_src: Arc<Mutex<dyn Unit>>,
    mux_mem_to_reg: Arc<Mutex<dyn Unit>>,
    mux_jr: Arc<Mutex<dyn Unit>>,
    alu_ctrl:Arc<Mutex<dyn Unit>>,
    reg_file:Arc<Mutex<dyn Unit>>,
    data_memory:Arc<Mutex<dyn Unit>>,
}


/// Control Implementation
impl<'a> Control {

    /// Returns a new Control, which will be able to set signals to Units given in arugments.
    ///
    ///  # Arguments
    /// 
    /// * `mux_reg_dst` - Mux to set
    /// * `mux_jump` - Mux to set
    /// * `ander_branch` - Ander to set
    /// * `mux_alu_src` - Mux to set
    /// * `mux_mem_to_reg` - Mux to set
    /// * `mux_jr` - Mux to set
    /// * `alu_ctrl` - ALU to set
    /// * `reg_file` - Register File to set
    /// * `data_memory` - Data Memory to set
    /// 
    /// # Returns
    ///
    /// * Control
    ///
    pub fn new(
        mux_reg_dst: Arc<Mutex<dyn Unit>>,
        mux_jump: Arc<Mutex<dyn Unit>>,
        ander_branch:Arc<Mutex<dyn Unit>>,
        mux_alu_src: Arc<Mutex<dyn Unit>>,
        mux_mem_to_reg: Arc<Mutex<dyn Unit>>,
        mux_jr: Arc<Mutex<dyn Unit>>,
        alu_ctrl:Arc<Mutex<dyn Unit>>,
        reg_file:Arc<Mutex<dyn Unit>>,
        data_memory:Arc<Mutex<dyn Unit>>,
    ) -> Control{
        Control{
            mux_reg_dst,
            mux_jump,
            mux_jr,
            ander_branch,
            mux_alu_src,
            mux_mem_to_reg,
            alu_ctrl,
            reg_file,
            data_memory,
        }
    }

    /// Sets all signals in the given Units to perform a R-type Instruction.
    pub fn set_r_signals(&mut self) {
        //println!("\t Controller sending r signals");
        // Signals that will be high
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, true);

        //Signals to be low
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform a Load Word Instruction.
    pub fn set_lw_signals(&mut self) {
        //println!("\t Controller sending lw signals");
        // Set alu src to high to change input to immediate value
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);
        // Set high to save data memory in register
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);
        // Set high to tell reg file to write to register
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the read signal
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, true);

         // Set all others signals to low
         self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
         self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
         self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);
         self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
         self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
         self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
         self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
         //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform a Store Word Instruction.
    pub fn set_sw_signals(&mut self) {
        //println!("\t Controller sending sw signals");
        // Set alu src to high to change input to immediate value
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, true);

        // Set all others signals to low
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform a Branch Equal Instruction.
    pub fn set_beq_signals(&mut self) {
        //println!("\t Controller sending beq signals");
        // Set singal to branch high
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, true);
        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, true);

        // Set all others signals to low
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform an Jump Instruction.
    pub fn set_j_signals(&mut self) {
        //println!("\t Controller sending j signals");
        // Set jump mux to high
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Set all others signals to low
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform an Add immediate Instruction.
    pub fn set_addi_signals(&mut self) {
        //println!("\t Controller sending addi signals");
        // Set alu input to immidiete
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Set write reg to I instruction
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);

        // Set reg file to write back
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Set all memory low
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);

        // Set ALU output insted of Data Memory
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);

        // Set branch and jump to low
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);

        // Set ALU Controler signals
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, false);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Sets all signals in the given Units to perform an Or immediate Instruction.
    pub fn set_ori_signals(&mut self) {
        //println!("\t Controller sending ori signals");
        // Set alu input to immidiete
        self.mux_alu_src.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Set reg file to write back
        self.reg_file.lock().unwrap().receive_signal(DEFAULT_SIGNAL, true);

        // Set write reg to I instruction
        self.mux_reg_dst.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);

        // Set all memory low
        self.data_memory.lock().unwrap().receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.lock().unwrap().receive_signal(MEM_READ_SIGNAL, false);

        // Set ALU output insted of Data Memory
        self.mux_mem_to_reg.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);

        // Set branch and jump to low
        self.mux_jump.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.lock().unwrap().receive_signal(BRANCH_SIGNAL, false);

        // Set ALU Controler signals
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.lock().unwrap().receive_signal(ALU_OP2_SIGNAL, true);
        //self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, false);
    }

    /// Set signal to chose address from register or using current address +4.
    /// 
    /// # Arguments
    /// 
    /// * `b` - Boolean to set the signal to
    pub fn set_jr_signal(&mut self, b: bool) {
        // Set jr mux to high to jump to value in register
        self.mux_jr.lock().unwrap().receive_signal(DEFAULT_SIGNAL, b);
    }
}

/// Control implementing Unit trait.
impl Unit for Control{

    /// Receives OP Code or Function code from a Unit, decides which signls to 
    /// set High or Low based on these.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive (&mut self, input_id : u32, data : Word) {
        // Check what type of data is comming 
       

         // Check if the data is funct code, if it is we a JR instruction is coming   
        if input_id == FUNCT_CONTROL {
            // JR instruction
            match data.to_bitvec().into_vec()[0] {
                0x08=>
                    self.set_jr_signal(true),
                _=>
                    self.set_jr_signal(false),
            }
        
        // If a OP code check what type of instruction
        }else if input_id == OP_CONTROL {
            //println!("\t Control received: {}",data);
            //println!("\t as u32: {:#032b}", data.to_bitvec().into_vec()[0]);
            
            match data.to_bitvec().into_vec()[0] {
                // R-format instructions 
                0b000000=> 
                    self.set_r_signals(),
                    // Set reg_dst, reg_wrt, alu_op1
                
                // LW instruction
                0b100011 =>
                    self.set_lw_signals(),
                    // Set alu_src, memto_reg, reg_wrt, mem_read, 

                // SW instruction
                0b101011 =>
                    self.set_sw_signals(),
                    // Set alu_src, mem_write

                // Beq instruction
                0b000100 =>
                    self.set_beq_signals(),
                    
                    // Set branch, alu_op0

                // Jump instruction
                0b000010 =>
                    self.set_j_signals(),
                    // Set jump

                // Addi instruction
                0b001000 =>
                    self.set_addi_signals(),

                // Ori instruction
                0b001101 =>
                    self.set_ori_signals(),

                //DO NOTHING
                _ =>(),
                 //DO NOTHING
            }
        }
    }

    /// Does nothing as the controller can not send signls to it self.
    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        // Does nothing
    }

    /// Does nothing, however need to be implemented as it exist in the Unit trait.
    fn execute(&mut self) {
        // Does Nothing
    }

}

