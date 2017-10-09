use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum EKeyCode {
    EKeyA,
    EKeyB,
    EKeyS,
    EKeyW,
    EKeyUpArrow,
    EKeyDownArrow,
}

pub struct Input {
    pub keys: HashMap<EKeyCode, bool>,
}

impl Input {
    pub fn new() -> Input {
        Input { keys: HashMap::new() }
    }

    pub fn poll(&mut self) {}

    pub fn is_space_down(&self) -> bool {
        true
    }

    pub fn get_key_down(&self, _key_code: EKeyCode) -> bool {
        return true;
    }
}
