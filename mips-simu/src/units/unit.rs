

mod unit {
    type Word = BitVec::<u32, Lsb0>;

    pub trait Unit:Default{
        fn receive(&self, input_id : u32, data :Word);
        fn receive_signal(&self ,signal_id:u32);

        fn default() ->Unit{
            EmptyUnit{
                
            }
        }
    }

    struct EmptyUnit{

    }

    impl Unit for EmptyUnit{
        fn receive(&self, input_id : u32, data :Word){
            println!("Empty Unit recived data: data = {}", data);
        }
    }

    const PC_IN_ID :u32 = 0;

    const IM_READ_ADDRESS_ID:u32  = 0;
    
    const REG_READ_1_ID:u32  = 0;
    const REG_READ_2_ID :u32 = 1;
    const REG_WRITE_DATA_ID:u32  = 2;
    const REG_WRITE_REG_ID :u32 = 3;

    const CTRL_IN_ID:u32  = 0;
    
    const ALU_CTRL_IN_ID :u32 = 0;

    const SE_IN_ID:u32  = 0;

    const AC_IN_ID:u32  = 0;

    const ALU_IN_1_ID:u32  = 0;
    const ALU_IN_2_ID :u32 = 1;

    const DM_ADDR_ID:u32  = 0;
    const DM_DATA_ID:u32  = 1;

    const MUX_IN_0_ID:u32  = 0;
    const MUX_IN_1_ID:u32  = 1;

    const CONC_IN_1_ID:u32  = 0;
    const CONC_IN_2_ID:u32  = 1;

    const ADD_IN_1_ID :u32 = 0;
    const ADD_IN_2_ID:u32  = 1;


    // Define control signals for data memory, since it has two signals
    const MEM_WRITE_SIGNAL:u32  = 0;
    const MEM_READ_SIGNAL:u32  = 1;
    // Define control signals for alu control op code, also tow signals
    const ALU_OP0_SIGNAL :u32 = 0;
    const ALU_OP1_SIGNAL:u32  = 1;
    // Define default signal const for all components with just one signal.
    const DEFAULT_SIGNAL:u32  = 0;    

    // Define ALU control signals
    const ALU_CTRL0_SIGNAL:u32  = 0;
    const ALU_CTRL1_SIGNAL:u32  = 1;
    const ALU_CTRL2_SIGNAL:u32  = 2;
    const ALU_CTRL3_SIGNAL:u32  = 3;
}