pub trait Unit{
    pub fn receive(&self, input_id : u32, data : Word);
    pub fn receive_signal(&self, signal_id: u32);
}

mod mux{
    use std::default;

    
    struct Mux {
        data0 : Word,
        data1 : Word,

        signal : bool,
        output_unit : &impl Unit,
        output_id : u32,

        has_val0 : bool,
        has_val1 : bool,
    }

    impl Mux {
  

        fn new(out: &impl Unit, out_id : u32) -> Mux{
            
            Mux{
                output_unit: Default::default(),
                output_id: 0,
                signal : false,
                data0: bitvec![u32, Lsb0; 0; 32],
                data1: bitvec![u32, Lsb0; 0; 32],
                has_val0: false,
                has_val1: false,
            }
        }

        fn execute(&self){
            // Some type of loop so the signal doesnt go unnoticed
            if signal{
                output_unit.receive(self.output_id, data1);
            }else{
                output_unit.receive(self.output_id, data0);
            }
        }
    }

    impl Unit for Mux{
        fn receive(&self, input_id: u32, data : BitVec::<u32, LocalBits> ){
            if input_id = MUX_IN_0_ID{
                data0 = data;
            }else if input_id = MUX_IN_1_ID {
                data1 = data;
            }else{
                //Data came on undefined input_id
            }
        }


        fn receive_signal(&self ,signal_id:u32){
            self.signal = true;
        }
    }


}









