

mod unit {
    type Word = BitVec::<u32, Lsb0>;

    pub trait Unit{
        fn receive(&self, input_id : u32, data :Word);
        fn receive_signal(&self ,signal_id:u32);
    }

    struct empty_unit{

    }

    impl Unit for empty_unit{
        fn receive(&self, input_id : u32, data :Word){
            println!("Empty Unit recived data: data = {}", data);
        }
    }

    const PC_IN_ID = 0;

    const IM_READ_ADDRESS_ID = 0;
    
    const REG_READ_1_ID = 0;
    const REG_READ_2_ID = 1;
    const REG_WRITE_DATA_ID = 2;
    const REG_WRITE_REG_ID = 3;

    const CTRL_IN_ID = 0;
    
    const ALU_CTRL_IN_ID = 0;

    const SE_IN_ID = 0;

    const AC_IN_ID = 0;

    const ALU_IN_1_ID = 0;
    const ALU_IN_2_ID = 1;

    const DM_ADDR_ID = 0;
    const DM_DATA_ID = 1;

    const MUX_IN_0_ID = 0;
    const MUX_IN_1_ID = 1;

    const CONC_IN_1_ID = 0;
    const CONC_IN_2_ID = 1;

    const ADD_IN_1_ID = 0;
    const ADD_IN_2_ID = 1;


    // Define control signals for data memory, since it has two signals
    const mem_write_signal = 0;
    const mem_read_signal = 1;
    // Define control signals for alu control op code, also tow signals
    const alu_op0_signal = 0;
    const alu_op1_signal = 1;
    // Define default signal const for all components with just one signal.
    const default_signal = 0;    

    // Define ALU control signals
    const alu_ctrl0_signal = 0;
    const alu_ctrl1_signal = 1;
    const alu_ctrl2_signal = 2;
    const alu_ctrl3_signal = 3;
}