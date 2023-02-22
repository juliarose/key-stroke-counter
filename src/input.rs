// Constants, structs, and arrays derived from /linux/include/linux/input.h

const EV_KEY: u16 = 1;
const KEY_PRESS: i32 = 1;

#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize, // from timeval struct
    tv_usec: isize, // from timeval struct
    pub r#type: u16,
    pub code: u16,
    pub value: i32
}

impl InputEvent {
    pub fn is_key_event(&self) -> bool {
        self.r#type == EV_KEY
    }
    
    pub fn is_key_press(&self) -> bool {
        self.value == KEY_PRESS
    }
}