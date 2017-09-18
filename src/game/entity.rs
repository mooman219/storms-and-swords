use cgmath::Vector3;
use game::World;
use graphics::render_thread::RenderFrame;


//it is this large for two reasons, one I want to make sure that we never run out to space, and second so that we can have negative uids for flag varibles
pub type UID = u64;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum EEntityType {
    PADDLE,
    BALL,
}

pub trait Entity {
  fn get_entity_type(&self) -> EEntityType;
  fn get_uid(&self) -> UID;
  fn add_to_render_frame(&self, render_frame: &mut RenderFrame);
}

pub trait EntityController {
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World)>>;
    fn get_entity_type(&self) -> EEntityType;
}

/*
pub struct Entity {
    name: String,
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID,
}


impl Entity {
    pub fn new(name: String, uid: UID) -> Entity {
        Entity {
            name: name,
            position: Vector3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
            scale: Vector3::<f32>::new(1.0f32, 1.0f32, 1.0f32),
            rotation: Vector3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
            uid: uid,
        }
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
*/
