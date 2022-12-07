
mod mux{
    
    struct mux {
        data0 : BitSlice::<LocalBits, usize>;
        data1 : BitSlice::<LocalBits, usize>;

        signal : bool;
        output_unit : &impl Unit;
        output_id : u32;

        has_val0 : bool;
        has_val1 : bool;
    }

    pub trait Mux{
        pub fn new(out: &impl Unit, out_id : u32) -> self; 
    }

    

    impl Mux for mux{
  

        pub fn new(out: &impl Unit, out_id : u32) -> mux{
            
            mux{
                output_unit: out,
                output_id: out_id,
                signal : false,
                output_unit : out,
            }
        }

        fn execute(&self){
            if signal{
                output_unit.receive(self.output_id, data1);
            }else{
                output_unit.receive(self.output_id, data0);
            }
        }
    }

    impl Unit for mux{
        pub fn receive(&self, input_id: u32, data : BitVec::<u32, LocalBits> ){
            if input_id = MUX_IN_0_ID{
                data0 = data;
            }else if input_id = MUX_IN_1_ID {
                data1 = data;
            }else{
                //Data came on undefined input_id
            }
        }


        pub fn receive_signal(&self ,signal_id:u32){
            self.signal = true;
        
        }


    }


}









