pub trait Unit{
    pub fn receive(&self, input_id : u32, data : BitSlice::<LocalBits, usize>;);
}

mod mux{
    
    struct mux {
        data0 : BitSlice::<LocalBits, usize>;
        data1 : BitSlice::<LocalBits, usize>;

        signal : bool;
        output_unit : &impl Unit;
        output_id : u32;

        input0_id : u32;
        input1_id : u32;

        has_val0 : bool;
        has_val1 : bool;
    }

    pub trait Mux{
        pub fn new(instr: Vec<u32>, out: &impl Unit) -> self; 
    }

    

    impl Mux for mux{
        //Define input-id´s
        const IN0_ID = 0;
        const IN1_ID = 1;

        pub fn new(instr: Vec<u32>, out: &impl Unit, out_id : u32) -> mux{
            
            mux{
                output_unit: out,
                output_id: out_id,
                signal : false,
                output_unit : out,
                input0_id : IN0_ID,
                input1_id : IN1_ID,
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
        pub fn receive(&self, input_id: u32, data : BitArr::<LocalBits, usize>){
            if input_id = &self.input0_id {
                data0 = data;
            }else if input_id = &self.input1_id {
                data1 = data;
            }else{
                //Data came on undefined input_id
            }
        }
    }


}









