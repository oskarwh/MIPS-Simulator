use bitvec::prelude::*;

mod sign_extend {

    struct sign_extend {

        data : Word;
        has_data: mut bool;
   
        add_unit : &impl Unit;

    }


    impl sign_extend{

        pub fn new() -> instruction_memory{
            sign_extend{
                has_data:false,
            }
        }


        ///Execute unit with thread
        pub fn execute(&self){

            if has_data{
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
        pub fn set_add(&self, add: &impl Unit;){
            self.add_unit = add;
        }

        ///Stop this thread
        pub fn stop(&self){
            stop =true;
        }


    }

    impl Unit for sign_extend{

        pub fn receive(&self, input_id: u32, data : Word){
            if input_id == SE_IN_ID{
                self.data = data;
                self.has_data = true;
            }else {
                //Unknown input-id
            }
        }

        
    }

}

