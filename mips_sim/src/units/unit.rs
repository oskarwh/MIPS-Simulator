

mod unit {
    type Word = BitVec::<u32, Lsb0>;

    pub trait Unit{
        pub fn receive(&self, input_id : u32, data :Word);
        pub fn receive_signal(&self ,signal_id:u32);
    }


    const PC_IN_ID = 0;

    const IM_READ_ADDRESS_ID = 0;
    
    const REG_READ_1_ID = 0;
    const REG_READ_2_ID = 1;
    const REG_WRITE_DATA_ID = 2;
    const REG_WRITE_REG_ID = 3;

    const CTRL_IN_ID = 0;

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


}