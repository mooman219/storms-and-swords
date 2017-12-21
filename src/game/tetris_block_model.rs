use cgmath::Vector3;
use game::entity::{Entity, UID, EEntityType, EntityController};
use game::world::World;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::f32;


#[derive(Copy, Clone)]
pub struct TetrisBlockModel {
    pub pos: Vector3<f32>,
    pub color: [f32;4],
    pub uid: UID,
    pub is_middle: bool
}

impl TetrisBlockModel {
    pub fn new(pos: Vector3<f32>, color: [f32;4], uid: UID, is_middle: bool) -> TetrisBlockModel {
        TetrisBlockModel {
            pos,
            color,
            uid,
            is_middle
        }
    }
}

impl Entity for TetrisBlockModel {
    fn get_entity_type(&self) -> EEntityType {
        EEntityType::TetrisBlock
    }

    fn set_uid(&mut self, uid: UID) {
        self.uid = uid;
    }

    fn get_uid(&self) -> UID {
        self.uid
    }

    fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
        if render_frame.sqaures.is_none() {
            render_frame.sqaures = Some(vec![]);
        }

        let sqd = SquareRenderData{
            pos: [self.pos.x, self.pos.y],
            height: 100.0f32,
            width: 100.0f32,
            color: [self.color[0], self.color[1], self.color[2]],
            use_border: true
        };

        render_frame.sqaures.as_mut().unwrap().push(sqd);
    }
} 