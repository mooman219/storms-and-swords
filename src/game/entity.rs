use ::game::component::Component;
use std::collections::HashMap;
use cgmath::Vector3;

pub struct Entity {
    pub name: String,
    pub components: HashMap<String, Box<Component>>,
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>
}

impl Entity {

    pub fn add_component(&mut self, component: Box<Component>) -> bool {
        if !self.components.contains_key(&component.get_name()) {
            self.components.insert(component.get_name(), component);
            return true;
        }
        return false;
    }

    pub fn remove_component(&mut self, name: String) {
        self.components.remove(&name);
    }

    
}