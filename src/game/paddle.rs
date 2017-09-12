use game::entity::{Entity, UID};
use cgmath::{Vector3, Vector2};
use game::ContentId;
use game::world::World;
use game::input::EKeyCode;

use graphics::BoxRenderData;

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
                    .clone() + Vector3::new(0f32, -0.001f32, 0.0f32);
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
                    .clone() + Vector3::new(0f32, 0.001f32, 0.0f32);
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

    pub fn move_pos_x(&mut self, shift_X: f32) {
        self.position = Vector3::new(self.position.x + shift_X, self.position.y, self.position.z);
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
