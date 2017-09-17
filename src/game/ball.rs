use cgmath::Vector2;
use graphics::sphere_renderer::SphereRenderData;
use game::World;
use game::entity::{Entity, EntityController, EEntityType, UID};

pub struct BallModel {
    pos: Vector2<f32>,
    uid: UID
}

impl BallModel {

    pub fn new(uid: UID) -> BallModel {
        BallModel {
            pos: Vector2::new(0.0f32, 0.0f32),
            uid
        }
    }

    pub fn get_sphere_render_data(&self) -> SphereRenderData {
        SphereRenderData {
            pos: self.pos.clone(),
            scale: 0.25f32,
            color: [1.0f32, 0.4f32, 0.1f32]
        }
    }
}

impl Entity for BallModel {
  fn get_entity_type(&self) -> EEntityType {
    EEntityType::BALL
  }

  fn get_uid(&self) -> UID {
    self.uid
  }
}

pub struct BallController {}

impl EntityController for BallController {
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World)>> {


        let return_function = move |inner_world: &mut World| {
            let uid_list = inner_world.type_to_uid_list[&EEntityType::BALL].clone();

            for uid in uid_list {

              let test = inner_world.get_mut_entity(uid);

              let test = match test {
                  Some(val) => val,
                  None => {
                    return;
                  },
              };


              let test = unsafe {&mut *(test as *mut &Entity as *mut &BallModel)};

            }

      };

        return Some(Box::new(return_function));
    }

    fn get_entity_type(&self) -> EEntityType {
        return EEntityType::BALL;
    }
}
