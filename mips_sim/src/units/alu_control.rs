mod unit;
use bitvec::prelude::*;

struct AluControl<'a> {
    alu_op0: bool,
    alu_op1: bool,

    alu_unit: &'a alu,
}

impl Unit for AluControl {
   fn new () -> AluControl{

    }

    fn execute() {
        
    }

    fn receive (&self, input_id : u32, data :BitVec::<u32, LocalBits>) {
        if(input_id == ALU_CTRL_IN_ID) {
            match data {
                bitvec![1,0,0,0,0,0] => 
                    // Add command
                    set_add_signals(),
            }
        }else {
            // Wrong ID
        }
        
    }

    fn set_add_signals() {
        // Send 0010 to ALU
        alu_unit.receive_signal(alu_ctrl2_signal);
    }

    fn set_sub_signals() {
        // Send 0110 to ALU
        alu_unit.receive_signal(alu_ctrl1_signal);
        alu_unit.receive_signal(alu_ctrl2_signal);
    }

    fn set_and_signals() {
        // Send 0000 to ALU
        // We set no signals
    }

    fn set_or_signals() {
        // Send 0001 to ALU
        alu_unit.receive_signal(alu_ctrl3_signal);
    }

    fn set_slt_signals() {
        // Send 0111 to ALU
        alu_unit.receive_signal(alu_ctrl1_signal);
        alu_unit.receive_signal(alu_ctrl2_signal);
        alu_unit.receive_signal(alu_ctrl3_signal);
    }

    fn set_lw_signals() {
        // Send 0010 to ALU
        alu_unit.receive_signal(alu_ctrl2_signal);
    }

    fn set_sw_signals() {
        // Send 0010 to ALU
        alu_unit.receive_signal(alu_ctrl2_signal);
    }

    fn set_bra_signals() {
        // Send 0110 to ALU
        alu_unit.receive_signal(alu_ctrl1_signal);
        alu_unit.receive_signal(alu_ctrl2_signal);
    }


   fn receive_signal(&self ,signal_id:u32) {
        if(signal_id == ALU_OP0_SIGNAL) {
            alu_op0 = true;
        }else if(signal_id == ALU_OP1_SIGNAL){
            alu_op1 = true;
        }
    }

    fn set_alu(alu: &Unit) {
        alu_unit = alu;
    }
}
