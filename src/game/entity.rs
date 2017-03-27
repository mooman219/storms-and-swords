use ::game::component::Component;
use std::collections::HashMap;
use cgmath::Vector3;

<<<<<<< HEAD
//it is this large for two reasons, one I want to make sure that we never run out to space, and second so that we can have negative uids for flag varibles
//type UID = i64;

pub struct Entity<'a>{
    name: String,
    components: HashMap<String, &'a Component>,
=======
pub struct Entity {
    name: String,
    components: HashMap<String, Box<Component>>,
>>>>>>> adb54650e7336494cd9d626caa517be110f8a56f
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>
}

impl<'a> Entity<'a> {

    pub fn add_component(&mut self, component: &'a Component) -> bool {
        if !self.components.contains_key(&component.get_name()) {

            self.components.insert(component.get_name(), component);
            return true;
        }
        return false;
    }

    pub fn remove_component(&mut self, name: String) {
        self.components.remove(&name);
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position.clone() 
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        self.rotation.clone()
    }
    
    pub fn get_scale(&self) -> Vector3<f32> {
        self.scale.clone()
    }
}