use game::entity::{Entity, UID, EEntityType, EntityController};
use cgmath::{Vector3, Vector2};
use game::world::World;
use game::input::EKeyCode;

use graphics::BoxRenderData;

pub struct PaddleController {}

impl PaddleController {
    pub fn new() -> PaddleController {
        PaddleController {}
    }
}

impl EntityController for PaddleController {
   fn update(&self, _world: &World) -> Option<Box<Fn(&mut World)>> {

        let return_closure = move |inner_world: &mut World| {
          let uid_list = inner_world.type_to_uid_list[&EEntityType::PADDLE].clone();

          for uid in uid_list {

            let test = inner_world.get_mut_entity(uid);

            let test = match test {
                Some(val) => val,
                None => {
                  return;
                },
            };


            let test = unsafe {&mut *(test as *mut &Entity as *mut &PaddleModel)};

        };
      };

        return Some(Box::new(return_closure));
    }

    fn get_entity_type(&self) -> EEntityType {
        return EEntityType::PADDLE;
    }
}


pub struct PaddleModel {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID,
}

impl PaddleModel {
    pub fn new(uid: UID) -> PaddleModel {
        PaddleModel {
            position: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            scale: Vector3::new(1.0f32, 1.0f32, 1.0f32),
            rotation: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            uid: uid,
        }
    }

    pub fn set_position(&mut self, new_pos: Vector3<f32>) {
        self.position = new_pos;
    }

    pub fn move_pos_x(&mut self, shift_x: f32) {
        self.position = Vector3::new(self.position.x + shift_x, self.position.y, self.position.z);
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn set_scale(&mut self, new_scale: Vector3<f32>) {
        self.scale = new_scale;
    }

    pub fn get_scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_rotation(&mut self, new_rot: Vector3<f32>) {
        self.rotation = new_rot;
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    pub fn get_uid(&self) -> UID {
        self.uid.clone()
    }

    pub fn get_box_render_data(&self) -> BoxRenderData {
        BoxRenderData {
            pos: Vector2::new(self.position.x, self.position.y),
            scale: Vector2::new(self.scale.x, self.scale.y),
            z_rotation: 0f32,
            color: [0.8f32, 0.4f32, 0.6f32]
        }
    }
}

impl Entity for PaddleModel {
  fn get_entity_type(&self) -> EEntityType {
    EEntityType::PADDLE
  }

  fn get_uid(&self) -> UID {
    self.uid
  }
}
