use ::game::component::Component;
use std::collections::HashMap;
use cgmath::Vector3;

//it is this large for two reasons, one I want to make sure that we never run out to space, and second so that we can have negative uids for flag varibles
pub type UID = i64;

pub struct Entity {
    name: String,
    components: HashMap<String, Box<Component>>,
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID
}

impl Entity {

    pub fn new(name: String, uid: UID) -> Entity {
        Entity{name: name,
               components: HashMap::new(),
               position: Vector3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
               scale: Vector3::<f32>::new(1.0f32, 1.0f32, 1.0f32),
               rotation: Vector3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
               uid: uid}
    }

    pub fn add_component(&mut self, component: Box<Component>) -> bool {
        if !self.components.contains_key(&component.get_name()) {

            self.components.insert(component.get_name(), component);
            return true;
        }
        return false;
    }

    pub fn get_component(&self, name: String) -> Option<&Box<Component>> {
       self.components.get(&name)
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
    
    pub fn get_uid(&self) -> UID {
        self.uid.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    } 
}