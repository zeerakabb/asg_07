
mod resturant_front_operations{
    pub  mod resturant{
       pub  fn add_to_wait_list(){
           println!("in the sub module og add_to _wait_list");
           //dowashing();
       }
         
     }
    }
       mod lib;
       use operations;
 fn main() {
    lib::resturant_with_background_operations::backround_operations::do_washing();
    operations::resturant::foods::orderFood();
 }