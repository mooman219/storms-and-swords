use ::game::component::Component;
use ::math::vector3::Vector3;
use std::collections::HashMap;

pub struct Entity {
    pub name: String,
    pub components: HashMap<String, Box<Component>>,
    pub position: Vector3,
    pub scale: Vector3,
    pub rotation: Vector3
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
     //   self.components.remove(name);
    }
}