// When doing sinlge instruction the contoler can sen out the signlas when it receives 
// the fucntion bits to all components in the data path. When a new instruction is loaded
// in we reset all signals and send the new ones. However when doing pipelining this needs 
// to be changed, the controler then needs to change specific singals depending on where 
// in the datapath a specific instruction is. How is this done?

// The booleans here can be removed but they are left if they maybe are need in the future
use crate::units::unit::*;
use bitvec::prelude::*;


pub struct Control<'a> {
    mux_reg_dst:  &'a mut dyn  Unit,
    mux_jump:  &'a mut dyn  Unit,
    ander_branch: &'a mut dyn  Unit,
    mux_alu_src:  &'a mut dyn  Unit,
    mux_mem_to_reg:  &'a mut dyn  Unit,
    mux_jr: &'a mut dyn Unit,
    alu_ctrl: &'a mut dyn  Unit,
    reg_file: &'a mut dyn  Unit,
    data_memory: &'a mut dyn  Unit,

    // Bit vector for R format instruction
    r_bitvec: BitVec<u32>, 
    // Bit vector for load-woard instruction
    lw_bitvec: BitVec<u32>, 
    // Bit vector for store-word instruction
    sw_bitvec: BitVec<u32>,
    // Bit vector for branch on equal instruction
    beq_bitvec: BitVec<u32>,
    // Bit vector for jump instruction
    j_bitvec: BitVec<u32>, 
    // Bit vector for addi instruction
    addi_bitvec: BitVec<u32>,
    // Bit vector for ori instruction
    ori_bitvec: BitVec<u32>,
}



impl Control<'_> {



    pub fn new<'a>(
        mux_reg_dst: &'a mut dyn  Unit,
        mux_jump: &'a mut dyn  Unit,
        mux_jr: &'a mut dyn Unit,
        ander_branch: &'a mut dyn  Unit,
        mux_alu_src:  &'a mut dyn  Unit,
        mux_mem_to_reg: &'a mut dyn  Unit,

        alu_ctrl: &'a mut dyn  Unit,
        reg_file: &'a mut dyn  Unit,
        data_memory:  &'a mut dyn  Unit,
    ) -> Control<'a>{
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

            r_bitvec: bitvec![u32, Lsb0; 0,0,0,0,0,0],
            lw_bitvec: bitvec![u32, Lsb0; 1,0,0,0,1,1],
            sw_bitvec: bitvec![u32, Lsb0; 1,0,1,0,1,1],
            beq_bitvec: bitvec![u32, Lsb0; 0,0,0,1,0,0],
            j_bitvec: bitvec![u32, Lsb0; 0,0,0,0,1,0],
            addi_bitvec:  bitvec![u32, Lsb0; 0,0,1,0,0,0],
            ori_bitvec: bitvec![u32, Lsb0; 0,0,1,1,0,1],
        }
    }

    pub fn set_r_signals(&mut self) {
        // Signals that will be high
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, true);
        self.reg_file.receive_signal(DEFAULT_SIGNAL, true);

        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, true);

        //Signals to be low
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_lw_signals(&mut self) {
        // Set alu src to high to change input to immediate value
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, true);
        // Set high to save data memory in register
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, true);
        // Set high to tell reg file to write to register
        self.reg_file.receive_signal(DEFAULT_SIGNAL, true);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the read signal
        self.data_memory.receive_signal(MEM_READ_SIGNAL, true);

         // Set all others signals to low
         self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);
         self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
         self.ander_branch.receive_signal(BRANCH_SIGNAL, false);
         self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
         self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
         self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
         self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
         self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_sw_signals(&mut self) {
        // Set alu src to high to change input to immediate value
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, true);
        // Since data mem has two signals we to define which signal to assert,
        // in this case it is the write signal
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, true);

        // Set all others signals to low
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);
        self.reg_file.receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_beq_signals(&mut self) {
        // Set singal to branch high
        self.ander_branch.receive_signal(DEFAULT_SIGNAL, true);
        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, true);

        // Set all others signals to low
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.reg_file.receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_j_signals(&mut self) {
        // Set jump mux to high
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, true);

        // Set all others signals to low
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);
        self.reg_file.receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }


    pub fn set_addi_signals(&mut self) {
        // Set alu input to immidiete
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, true);

        // Set write reg to I instruction
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);

        // Set reg file to write back
        self.reg_file.receive_signal(DEFAULT_SIGNAL, true);

        // Set all memory low
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);

        // Set ALU output insted of Data Memory
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);

        // Set branch and jump to low
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);

        // Set ALU Controler signals
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_ori_signals(&mut self) {
        // Set alu input to immidiete
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, true);

        // Set reg file to write back
        self.reg_file.receive_signal(DEFAULT_SIGNAL, true);

        // Set write reg to I instruction
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, false);

        // Set all memory low
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);

        // Set ALU output insted of Data Memory
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);

        // Set branch and jump to low
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);

        // Set ALU Controler signals
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, true);
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, false);
    }

    pub fn set_jr_signals(&mut self) {
        // Signals that will be high
        self.mux_reg_dst.receive_signal(DEFAULT_SIGNAL, true);

        // Set jr mux to high to jump to value in register
        self.mux_jr.receive_signal(DEFAULT_SIGNAL, true);

        // Since alu ctrl has two signals we have to define which signal to assert.
        self.alu_ctrl.receive_signal(ALU_OP1_SIGNAL, true);

        //Signals to be low
        self.mux_alu_src.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_mem_to_reg.receive_signal(DEFAULT_SIGNAL, false);
        self.data_memory.receive_signal(MEM_READ_SIGNAL, false);
        self.data_memory.receive_signal(MEM_WRITE_SIGNAL, false);
        self.ander_branch.receive_signal(BRANCH_SIGNAL, false);
        self.reg_file.receive_signal(DEFAULT_SIGNAL, false);
        self.mux_jump.receive_signal(DEFAULT_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP0_SIGNAL, false);
        self.alu_ctrl.receive_signal(ALU_OP2_SIGNAL, false);
    }
}

impl Unit for Control<'_>{

    fn receive (&mut self, input_id : u32, data : Word) {
        // Check what type of data is comming 
        // If a new op_code check what type of instruction
        if input_id == OP_CONTROL {
            
            match data{
                // R-format instructions 
                r_bitvec =>
                    self.set_r_signals(),
                    // Set reg_dst, reg_wrt, alu_op1

                // LW instruction
                lw_bitvec =>
                    self.set_lw_signals(),
                    // Set alu_src, memto_reg, reg_wrt, mem_read, 

                // SW instruction
                sw_bitvec =>
                    self.set_sw_signals(),
                    // Set alu_src, mem_write

                // Beq instruction
                beq_bitvec =>
                    self.set_beq_signals(),
                    // Set branch, alu_op0

                // Jump instruction
                j_bitvec =>
                    self.set_j_signals(),
                    // Set jump

                // Addi instruction
                addi_bitvec =>
                    self.set_addi_signals(),

                // Ori instruction
                ori_bitvec =>
                    self.set_ori_signals(),
            }
        // Check if the data is funct code, if it is we a JR instruction is coming   
        } else if input_id == FUNCT_CONTROL {
            // JR instruction

        }
    }

    fn receive_signal(&mut self ,signal_id:u32, signal: bool) {
        // Does nothing
    }

}
