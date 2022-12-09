use crate::unit::*;
use bitvec::prelude::*;

mod unit;
mod mux;

struct SignExtend<'a>  {

    data : Word,
    has_data: bool,

    add_unit : &'a dyn Unit,

}


impl SignExtend<'_>{

    pub fn new() -> SignExtend<'static>{
        SignExtend{
            has_data:false,
            data: bitvec![u32, Lsb0; 0; 32],
            add_unit: &EmptyUnit{},
        }
    }


    ///Execute unit with thread
    pub fn execute(&self){

        if self.has_data{
            //Sign extend the data
            let sign = self.data[15];

            for _ in 16..32{
                self.data.push(sign);
            }
            //Shift the data left (shift_right because of the way BitVec is designed)
            self.data.shift_right(2);

            self.add_unit.receive(ADD_IN_2_ID, self.data.to_bitvec());
        
        }
    }

    /// Set Functions
    pub fn set_add(&self, add: &impl Unit){
        self.add_unit = add;
    }



}

impl Unit for SignExtend<'_>{

    fn receive(&self, input_id: u32, data : Word){
        if input_id == SE_IN_ID{
            self.data = data;
            self.has_data = true;
        }else {
            //Unknown input-id
        }
    }

    fn receive_signal(&self ,signal_id:u32) {
        todo!()
    }


}



