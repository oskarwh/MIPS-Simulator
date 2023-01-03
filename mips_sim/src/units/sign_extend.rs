use bitvec::prelude::*;
use crate::units::unit::*;
use std::sync::Mutex;
use std::sync::Arc;
use super::mux::Mux;

/// A MIPS simulator unit. SignExtend will take 16 bit integer and extend it to 32 bits,
///
/// Authors: Jakob Lindehag (c20jlg@cs.umu.se)
///          Oskar Westerlund Holmgren (c20own@cs.umu.se)
///          Max Thorén (c20mtn@cs.umu.se)
///
/// Version information:
///    v1.0 2022-01-03: First complete version.
/// 


/// SignExtend Struct
pub struct SignExtend {
    data : Word,
    has_data: bool,

    add_unit : Option<Arc<Mutex<dyn Unit>>>,
    mux_alusrc : Option<Arc<Mutex<Mux>>>,
}

/// SignExtend Implementation
impl SignExtend{

    /// Returns a new SignExtend.
    /// 
    /// # Returns
    ///
    /// * SignExtend
    ///
    pub fn new() -> SignExtend{
        SignExtend{
            has_data:false,
            data: bitvec![u32, Lsb0; 0; 32],
            add_unit: None,
            mux_alusrc: None,
        }
    }


    /// Set a AddUnit that the 'SignExtend' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `add` - The AddUnit that should be set
    ///
    pub fn set_add(&mut self, add: Arc<Mutex<dyn Unit>>){
        self.add_unit = Some(add);
    }

    /// Set which Mux that the 'SignExtend' which is called on, should send data to.
    /// 
    /// # Arguments
    ///
    /// * `mux` - The Mux that should be set
    ///
    pub fn set_mux_alu_src(&mut self, mux: Arc<Mutex<Mux>>){
        self.mux_alusrc = Some(mux);
    }

}

/// SignExtend implementing Unit trait.
impl Unit for SignExtend{

    /// Receives data from a Unit, comes with ID to 
    /// specify which type of data.
    /// 
    /// # Arguments
    /// 
    /// * `input_id` - Id to know what type of data is comming
    /// * `data` - The data
    /// 
    fn receive(&mut self, input_id: u32, data : Word){
        if input_id == SE_IN_ID{
            self.data = data;
            self.has_data = true;
        }else {
            //Unknown input-id
        }
    }

    /// Receives signal from a Control, comes with ID to 
    /// specify which signal.
    /// 
    /// # Arguments
    /// 
    /// * `signal_id` - Id to know what type of signal is comming
    /// * `signal` - Bool which holds state of signal (high/low)
    /// 
    fn receive_signal(&mut self ,signal_id:u32, signal:bool) {
        // Do Nothing
    }

    /// Checks if data has been received.
    /// If that is the case extend the 16 bit data to 32 bits and forward.
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



