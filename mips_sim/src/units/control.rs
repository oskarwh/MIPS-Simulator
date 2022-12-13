// When doing sinlge instruction the contoler can sen out the signlas when it receives 
// the fucntion bits to all components in the data path. When a new instruction is loaded
// in we reset all signals and send the new ones. However when doing pipelining this needs 
// to be changed, the controler then needs to change specific singals depending on where 
// in the datapath a specific instruction is. How is this done?

// The booleans here can be removed but they are left if they maybe are need in the future
use bitvec::prelude::*;
use crate::units::unit::*;
use crate::units::mux::*;


pub struct Control<'a> {
    /* reg_dst: bool = false;
    reg_write: bool = false;
    
    alu_src: bool = false;

    branch: bool = false;
    jump: bool = false;
    
    mem_read: bool = false;
    memto_reg: bool = false;
    
    alu_op0: bool = false;
    alu_op1: bool = false;*/

    mux_reg_dst:  &'a dyn  Unit,
    mux_jump:  &'a dyn  Unit,
    mux_branch: &'a dyn  Unit,
    mux_alu_src:  &'a dyn  Unit,
    mux_mem_to_reg:  &'a dyn  Unit,
    alu_ctrl: &'a dyn Unit,
    reg_file: &'a dyn Unit,
    data_memory: &'a dyn Unit
}


impl Control<'_> {

    pub fn new<'a>(
        mux_reg_dst: &'a dyn  Unit,
        mux_jump: &'a dyn  Unit,
        mux_branch: &'a dyn  Unit,
        mux_alu_src:  &'a dyn  Unit,
        mux_mem_to_reg: &'a dyn  Unit,
        mux_mem:  &'a dyn  Unit,

        alu_ctrl: &impl Unit,
        reg_file: &impl Unit,
        data_memory:  &impl Unit,
    ) -> Control<'a>{
        Control{
            mux_reg_dst,
            mux_jump,
            mux_branch,
            mux_alu_src,
            mux_mem_to_reg,
            alu_ctrl,
            reg_file,
            data_memory
        }
    }

    pub fn set_r_signals(&mut self) {
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL);
        self.reg_file.receive_signal(DEFAULT_SIGNAL);
        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL);
    }

    pub fn set_lw_signals(&mut self) {
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL);
        self.reg_file.receive_signal(DEFAULT_SIGNAL);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the read signal
        self.data_memory.receive_signal(MEM_READ_SIGNAL);
    }

    pub fn set_sw_signals(&mut self) {
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL)
    }

    pub fn set_beq_signals(&mut self) {
        self. mux_alu_src.receive_signal(DEFAULT_SIGNAL);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL);
        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL);
    }

    pub fn set_j_signals(&mut self) {
        self.mux_jump.receive_signal(DEFAULT_SIGNAL);
    }

   /* // Reset all outoing signals
    fn reset_signals() {
        reg_dst: bool = false;
        reg_write: bool = false;
        alu_src: bool = false;
        branch: bool = false;
        jump: bool = false;
        mem_read: bool = false;
        memto_reg: bool = false;
        alu_op0: bool = false;
        alu_op1: bool = false;
    }*/
}

impl Unit for Control<'_>{

    fn receive (&mut self, input_id : u32, data : Word) {
        // Bit vector for R format instruction
        let r_bitvec = bitvec![0,0,0,0,0,0];
        // Bit vector for load-woard instruction
        let lw_bitvec = bitvec![1,0,0,0,1,1];
        // Bit vector for store-word instruction
        let sw_bitvec = bitvec![1,0,1,0,1,1];
        // Bit vector for branch on equal instruction
        let beq_bitvec = bitvec![0,0,0,1,0,0];
        // Bit vector for jump instruction
        let j_bitvec = bitvec![0,0,0,0,1,0];


        match data {

            // R-format instructions 
            r_bitvec =>  
                self.set_r_signals(),
                
                // Set reg_dst, reg_wrt, alu_op1
                /* reg_dst = true;
                reg_write = true;
                alu_op1 = true,*/
 
            // LW instruction
            lw_bitvec =>
                self.set_lw_signals(),
                // Set alu_src, memto_reg, reg_wrt, mem_read, 
                /*alu_src = true;
                memto_reg = true;
                reg_write = true;
                mem_read = true;*/

            // SW instruction
            sw_bitvec =>
                self.set_sw_signals(),
                // Set alu_src, mem_write
                /*alu_src = true;
                mem_write = true;*/

            // Beq instruction
            beq_bitvec =>
                self.set_beq_signals(),
                // Set branch, alu_op0
                /*branch = true;
                alu_op0 = true;*/

            // Jump instruction
            j_bitvec =>
                self.set_j_signals(),
                // Set jump bool 
                //jump = true;
        }    
    }

    fn receive_signal(&mut self ,signal_id:u32) {
        todo!()
    }

}
