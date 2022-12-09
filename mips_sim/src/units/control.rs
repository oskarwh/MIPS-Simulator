// When doing sinlge instruction the contoler can sen out the signlas when it receives 
// the fucntion bits to all components in the data path. When a new instruction is loaded
// in we reset all signals and send the new ones. However when doing pipelining this needs 
// to be changed, the controler then needs to change specific singals depending on where 
// in the datapath a specific instruction is. How is this done?

// The booleans here can be removed but they are left if they maybe are need in the future
mod mux;
mod unit;

use unit::Unit;
use mux::Mux;

struct control {
    /* reg_dst: bool = false;
    reg_write: bool = false;
    
    alu_src: bool = false;

    branch: bool = false;
    jump: bool = false;
    
    mem_read: bool = false;
    memto_reg: bool = false;
    
    alu_op0: bool = false;
    alu_op1: bool = false;*/

    mux_reg_dst: &impl Mux,
    mux_jump: &impl Mux,
    mux_branch: &impl Mux,
    mux_alu_src: &impl Mux,
    mux_mem_to_reg: &impl Mux,
    alu_ctrl: &impl Unit,
    reg_file: &impl Unit,
    data_memory: &impl Unit
}


impl Unit for control {

    fn new(
        mux_reg_dst: &impl Mux,
        mux_jump: &impl Mux,
        mux_branch: &impl Mux,
        mux_alu_src: &impl Mux,
        mux_mem: &impl Mux,

        alu_ctrl: &impl alu_control,
        reg_file: &impl register_file,
        data_memory: &impl Mux,
    ) -> control{
        control{
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


    fn receive (&self, input_id : u32, data : BitVec::<LocalBits, usize>) {
        match data {

            // R-format instructions 
            bits![0,0,0,0,0,0] =>  
                set_r_signals(),
                
                // Set reg_dst, reg_wrt, alu_op1
                /* reg_dst = true;
                reg_write = true;
                alu_op1 = true,*/
                
            // LW instruction
            bits![1,0,0,0,1,1] =>
                set_lw_signals(),
                // Set alu_src, memto_reg, reg_wrt, mem_read, 
                /*alu_src = true;
                memto_reg = true;
                reg_write = true;
                mem_read = true;*/

            // SW instruction
            bits![1,0,1,0,1,1] =>
                set_sw_signals(),
                // Set alu_src, mem_write
                /*alu_src = true;
                mem_write = true;*/

            // Beq instruction
            bits![0,0,0,1,0,0] =>
                set_beq_signals(),
                // Set branch, alu_op0
                /*branch = true;
                alu_op0 = true;*/

            // Jump instruction
            bits![0,0,0,0,1,0] =>
                set_j_signals(),
                // Set jump bool 
                //jump = true;
        }    
    }

    fn set_r_signals() {
        mux_reg_dst.receive_signal(default_signal);
        reg_file.receive_signal(default_signal);
        // Since alu ctrl has two signals we have to define which signal to assert.
        alu_ctrl.receive_signal(alu_op1_signal);
    }

    fn set_lw_signals() {
        mux_alu_src.receive_signal(default_signal);
        mux_mem_to_reg.receive_signal(default_signal);
        reg_file.receive_signal(default_signal);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the read signal
        data_memory.receive_signal(mem_read_signal);
    }

    fn set_sw_signals() {
        mux_alu_src.receive_signal(default_signal);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        data_memory.receive_signal(mem_write_signal)
    }

    fn set_beq_signals() {
        mux_alu_src.receive_signal(default_signal);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        data_memory.receive_signal(mem_write_signal);
        // Since alu ctrl has two signals we have to define which signal to assert.
        alu_ctrl.receive_signal(alu_op0_signal);
    }

    fn set_j_signals() {
        mux_jump.receive_signal(default_signal);
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
