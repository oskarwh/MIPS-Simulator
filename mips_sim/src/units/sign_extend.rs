use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Mutex;
pub struct SignExtend<'a>  {

    data : Word,
    has_data: bool,

    add_unit : Option<&'a Mutex<&'a mut dyn Unit>>,

}


impl<'a> SignExtend<'a>{

    pub fn new() -> SignExtend<'static>{
        SignExtend{
            has_data:false,
            data: bitvec![u32, Lsb0; 0; 32],
            add_unit: None,
        }
    }


    ///Execute unit with thread
    pub fn execute(&mut self){

        if self.has_data{
            //Sign extend the data
            let sign = self.data[15];

            for _ in 16..32{
                self.data.push(sign);
            }
            //Shift the data left (shift_right because of the way BitVec is designed)
            self.data.shift_right(2);

            self.add_unit.as_mut().unwrap().lock().unwrap().receive(ADD_IN_2_ID, self.data.to_bitvec());
        
        }
    }

    /// Set Functions
    pub fn set_add(&'a mut self, add: &'a Mutex<&'a mut dyn Unit>){
        self.add_unit = Some(add);
    }



}

impl Unit for SignExtend<'_>{

    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == SE_IN_ID{
            self.data = data;
            self.has_data = true;
        }else {
            //Unknown input-id
        }
    }

    fn receive_signal(&mut self ,signal_id:u32, signal:bool) {
        todo!()
    }


}



