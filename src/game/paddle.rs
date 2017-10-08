use game::entity::{Entity, UID, EEntityType, EntityController};
use cgmath::{Vector3, Vector2};
use game::world::World;
use graphics::renderer::RenderFrame;

use graphics::square_renderer::SquareRenderData;

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

            let test = unsafe {&mut *(test as *mut &Entity as *mut &mut PaddleModel)};
            let pos_x = test.get_position().x;

           test.set_position(Vector3::new(pos_x + 0.01f32, 0.0f32, 0.0f32));

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
            position: Vector3::new(0.001f32, 0.0f32, 0.0f32),
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
        self.scale = Vector3::new(new_scale.x * 1000f32, new_scale.y * 1000f32, new_scale.z * 1000f32);
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
}

impl Entity for PaddleModel {
  fn get_entity_type(&self) -> EEntityType {
    EEntityType::PADDLE
  }

  fn get_uid(&self) -> UID {
    self.uid
  }

  fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
      

      let srd = SquareRenderData {
          pos: [self.position.x, self.position.y],
          width: 100.0,
          height: 100.0,
          color: [0.8, 0.6, 0.7]
      };

    if render_frame.sqaures.is_none() {
      render_frame.sqaures = Some(vec![srd]);
    }
    else {
      render_frame.sqaures.as_mut().unwrap().push(srd);
    }
  }
}
