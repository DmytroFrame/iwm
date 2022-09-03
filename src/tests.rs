static mut GLOBAL: i32 = 0;

pub fn get() -> i32 {
    unsafe {
        return GLOBAL;
    }
}

pub fn set(numb: i32) -> i32 {
    unsafe {
        GLOBAL = numb;
        return GLOBAL;
    }
}
