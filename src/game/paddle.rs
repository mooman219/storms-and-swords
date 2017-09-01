use game::entity::{Entity, UID};
use cgmath::Vector3;
use game::ContentId;
use game::world::World;
use game::input::EKeyCode;
use graphics::{StaticSprite, SpriteRenderData};

pub struct PaddleController {}

impl PaddleController {
    pub fn new() -> PaddleController {
        PaddleController {}
    }

    pub fn update(&self, world: &World) -> Option<Box<Fn(&mut World)>> {

        let return_closure = move |inner_world: &mut World| {

            if inner_world.get_input().get_key_down(EKeyCode::EKeyS) {

                let new_pos = inner_world
                    .left_paddle
                    .as_ref()
                    .unwrap()
                    .get_position()
                    .clone() + Vector3::new(0f32, -1.0f32, 0.0f32);
                inner_world.left_paddle.as_mut().unwrap().set_position(
                    new_pos,
                );
            }

            if inner_world.get_input().get_key_down(EKeyCode::EKeyW) {
                let new_pos = inner_world
                    .left_paddle
                    .as_ref()
                    .unwrap()
                    .get_position()
                    .clone() + Vector3::new(0f32, 1.0f32, 0.0f32);
                inner_world.left_paddle.as_mut().unwrap().set_position(
                    new_pos,
                );
            }
        };

        return Some(Box::new(return_closure));
    }
}

pub struct PaddleModel {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID,
    sprite_id: ContentId,
}

impl PaddleModel {
    pub fn new(uid: UID, sprite_id: ContentId) -> PaddleModel {
        PaddleModel {
            position: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            scale: Vector3::new(1.0f32, 1.0f32, 1.0f32),
            rotation: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            uid: uid,
            sprite_id: sprite_id,
        }
    }

    pub fn set_position(&mut self, new_pos: Vector3<f32>) {
        self.position = new_pos;
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
}

impl StaticSprite for PaddleModel {
    fn generate_sprite_render_data(&self) -> SpriteRenderData {
        SpriteRenderData {
            pos: self.position.clone(),
            scale: self.scale.clone(),
            rotation: self.rotation.clone(),
            sprite: self.sprite_id.clone(),
        }
    }
}
