use std::collections::HashMap;
use graphics::renderer::{BASE_SCREEN_HEIGHT, BASE_SCREEN_WIDTH};
use glutin::{self, KeyboardInput, VirtualKeyCode};
#[derive(Eq, Copy, Clone, Hash, PartialEq, Debug)]
pub enum KeyState {
    Pressed,
    HeldBuffer,//there is a small dealy between holding down a key and its repeat speed, so we keys in this state for that period
    Released, 
    Held,
    Idle
}

pub enum InputMessage {
    KeyboardEvent(KeyboardInput),
    CursorEvent((f64, f64))
}

pub struct Input {
    key_state: HashMap<VirtualKeyCode, KeyState>,
    current_mouse_pos: (f64, f64)
}

impl Input {
    pub fn new() -> Input {
        Input { 
            key_state: HashMap::new(),
            current_mouse_pos: (0f64, 0f64)
        }
    }

    pub fn get_current_mouse_pos(&self) -> (f64, f64) {
        self.current_mouse_pos
    }

    pub fn get_key_down(_virtual_key_code: KeyboardInput) -> bool {
        
        return false;
    }

    //this should only return true if the key in question has been pressed in the same frame as it is being tested in
    pub fn on_key_pressed(&self, virtual_key_code: VirtualKeyCode) -> bool {
        
        if !self.key_state.contains_key(&virtual_key_code) {
            return false;
        }
        return self.key_state.get(&virtual_key_code).unwrap() == &KeyState::Pressed;
    }

    pub fn on_key_held(&self, virtual_key_code: VirtualKeyCode) -> bool {
        
        if !self.key_state.contains_key(&virtual_key_code) {
            return false;
        }
        return self.key_state.get(&virtual_key_code).unwrap() == &KeyState::Held;
    }

    pub fn on_key_released(&self, virtual_key_code: VirtualKeyCode) -> bool {
        if !self.key_state.contains_key(&virtual_key_code) {
            return false;
        }
        return self.key_state.get(&virtual_key_code).unwrap() == &KeyState::Released;
    }

    //there is some record keeping that the input system must do each frame, it is taken care of in this function
    //best called after everything in a frame that needs input has been called
    pub fn end_of_frame_clean(&mut self) {
        for val in self.key_state.values_mut() {
            if *val == KeyState::Pressed {
                *val = KeyState::HeldBuffer;
            }
            else if *val == KeyState::Released {
                *val = KeyState::Idle;
            }
        }
    }

    pub fn process_event(&mut self, new_event: InputMessage) {
        match new_event {
            InputMessage::CursorEvent(pos) => {
               self.process_mouse_input(pos);
            },
            InputMessage::KeyboardEvent(event) => {
                self.process_key_event(event);
            }
        }
    }

    pub fn process_mouse_input(&mut self, mut new_mouse_pos: (f64, f64)) {
        new_mouse_pos.0 = new_mouse_pos.0 / BASE_SCREEN_WIDTH as f64;
        new_mouse_pos.1 = 1.0 - (new_mouse_pos.1 / BASE_SCREEN_HEIGHT as f64);
        self.current_mouse_pos = new_mouse_pos;
    }

    //this function is the main ingestion for the input
    //the rules for input state are as follows
    //you may only ever be Idle, Held, Pressed, Or Released
    //you may only be Pressed, or Released for a single frame
    //between pressed and held there is held buffer, which isnt either, and that is important
    //it means that a key can fire off a "Pressed" event, but then wait until the key repeat speed to say "Held"
    //You are Held after being Pressed twice before hearing a released event
    //you are idle if it has been more then a single frame since you have been released
    pub fn process_key_event(&mut self, input_event: KeyboardInput) {
        if !self.key_state.contains_key(&input_event.virtual_keycode.as_ref().unwrap()) {
            self.key_state.insert(input_event.virtual_keycode.unwrap(), KeyState::Idle);
        }

        let current_state = self.key_state[&input_event.virtual_keycode.unwrap()];

        //the user has just struck the key and the main thread has processed the event
        if input_event.state == glutin::ElementState::Pressed {
            
            let val = self.key_state.get_mut(&input_event.virtual_keycode.unwrap()).unwrap();
         
            if *val == KeyState::Pressed {
                *val = KeyState::HeldBuffer;//the state between a key being pressed, and its passing its repeat threashold
            }
            else if *val != KeyState::Held && *val != KeyState::HeldBuffer {
                *val = KeyState::Pressed;
            }
            else if *val == KeyState::HeldBuffer {
                *val = KeyState::Held;
            }
        }
        else if input_event.state == glutin::ElementState::Released {
            let val = self.key_state.get_mut(&input_event.virtual_keycode.unwrap()).unwrap();
            match current_state {
                KeyState::Pressed => {
                    *val = KeyState::Released;
                },
                KeyState::Released => {
                    //this is an impossible state, something has gone wrong
                },
                KeyState::Idle => {
                    //this is an impossible state, something has gone wrong
                },
                KeyState::Held => {  
                    *val = KeyState::Released;
                },
                KeyState::HeldBuffer => {
                    *val = KeyState::Released;
                }
            }
        }

    }
}
