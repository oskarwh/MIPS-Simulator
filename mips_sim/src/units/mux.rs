
use crate::units::unit::*;
use bitvec::prelude::*;

pub struct Mux<'a> {
    data0 : Word,
    data1 : Word,

    signal : bool,
    output_unit : &'a mut dyn  Unit,
    output_id : u32,

    has_val0 : bool,
    has_val1 : bool,
}

impl Mux<'_> {


    pub fn new(out: &mut dyn  Unit, out_id : u32) -> Mux<'_>{
        
        Mux{
            output_unit: out,
            output_id: out_id,
            signal : false,
            data0: bitvec![u32, Lsb0; 0; 32],
            data1: bitvec![u32, Lsb0; 0; 32],
            has_val0: false,
            has_val1: false,
        }
    }

    pub fn execute(&mut self){
        // Some type of loop so the signal doesnt go unnoticed
        if self.signal{
            self.output_unit.receive(self.output_id, self.data1.to_bitvec());
        }else{
            self.output_unit.receive(self.output_id, self.data0.to_bitvec());
        }
    }

}

impl Unit for Mux<'_>{
    fn receive(&mut self, input_id: u32, data : BitVec::<u32, LocalBits> ){
        if input_id == MUX_IN_0_ID{
            self.data0 == data;
        }else if input_id == MUX_IN_1_ID {
            self.data1 = data;
        }else{
            //Data came on undefined input_id
        }
    }


    fn receive_signal(&mut self ,signal_id:u32){
        self.signal = true;
    }
}












