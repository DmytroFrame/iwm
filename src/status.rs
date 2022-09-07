static mut ONLINE: i32 = 0;

pub mod online {
    use super::ONLINE;

    pub fn get() -> i32 {
        unsafe { ONLINE }
    }

    pub fn plus() -> i32 {
        unsafe {
            ONLINE += 1;
            ONLINE
        }
    }

    pub fn minus() -> i32 {
        unsafe {
            ONLINE -= 1;
            ONLINE
        }
    }
}
