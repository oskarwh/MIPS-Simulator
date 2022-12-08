mod alu_control {

    struct alu_control {
        alu_op0: bool;
        alu_op1: bool;

        alu_unit: alu;
    }

    impl Unit for alu_control {
        // Define control signals for alu control op code, also tow signals
        const alu_op0 = 0;
        const alu_op1 = 1;

        pub fn new () -> alu_control{

        }

        pub fn execute

        pub fn receive (&self, input_id : u32, data :BitVec::<u32, LocalBits>) {
            if(input_id == ALU_CTRL_IN_ID) {
                match data 
                    => bitvec![1,0,0,0,0,0]
                        // Add command
                        set_add_signals():

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


        pub fn receive_signal(&self ,signal_id:u32) {
            if(signal_id == alu_op0_signal) {
                alu_op0 = true;
            }else if(signal_id == alu_op1_signal){
                alu_op1 = true;
            }
        }

        pub fn set_alu(alu: &Unit) {
            alu_unit = alu;
        }
    }
}