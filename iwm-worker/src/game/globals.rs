use std::sync::Mutex;





pub static mut MAX_ONLINE: Mutex<i32> = Mutex::new(0);
pub static mut CURRENT_ONLINE: Mutex<i32> = Mutex::new(0);
pub static mut VIEW_DISTANCE: Mutex<i32> = Mutex::new(0);
pub static mut SIMULATION_DISTANCE: Mutex<i32> = Mutex::new(0);
