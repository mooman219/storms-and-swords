use std::collections::HashMap;

use glutin::VirtualKeyCode;

pub struct Input {
    pub keys: HashMap<VirtualKeyCode, bool>,
}

impl Input {
    pub fn new() -> Input {
        Input { keys: HashMap::new() }
    }

    pub fn get_key_down(_virtual_key_code: VirtualKeyCode) -> bool {
        
        return false;
    }
}
