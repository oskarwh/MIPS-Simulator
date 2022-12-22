use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Mutex;
use std::sync::Arc;

use super::mux::Mux;

pub struct SignExtend {

    data : Word,
    has_data: bool,

    add_unit : Option<Arc<Mutex<dyn Unit>>>,
    mux_alusrc : Option<Arc<Mutex<Mux>>>,
}


impl SignExtend{

    pub fn new() -> SignExtend{
        SignExtend{
            has_data:false,
            data: bitvec![u32, Lsb0; 0; 32],
            add_unit: None,
            mux_alusrc: None,
        }
    }


    /// Set Functions
    pub fn set_add(&mut self, add: Arc<Mutex<dyn Unit>>){
        self.add_unit = Some(add);
    }

    pub fn set_mux_alu_src(&mut self, mux: Arc<Mutex<Mux>>){
        self.mux_alusrc = Some(mux);
    }

}

impl Unit for SignExtend{

    fn receive(&mut self, input_id: u32, data : Word){
        println!("\t Sign extend received {}", data);
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

    ///Execute unit with thread
    fn execute(&mut self){

        if self.has_data{
            //Sign extend the data
            let sign = self.data[15];

            for _ in 16..32{
                self.data.push(sign);
            }
            //Shift the data left fro add unit (shift_right because of the way BitVec is designed)
            let mut data_shifted = self.data.to_bitvec();
            data_shifted.shift_right(2);

            self.add_unit.as_mut().unwrap().lock().unwrap().receive(ADD_IN_2_ID, data_shifted);
            self.mux_alusrc.as_mut().unwrap().lock().unwrap().receive(MUX_IN_1_ID, self.data.to_bitvec());
            self.has_data = false;
        }
    }


}



