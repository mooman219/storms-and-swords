use game::entity::{Entity, UID, EEntityType};
use game::ui::UIEntity;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use cgmath::Vector3;

pub struct ScreenBlockers {
    pub uid: UID,
    pub pos: Vector3<f32>
}

impl ScreenBlockers {
    pub fn new(uid: UID) -> ScreenBlockers {
        ScreenBlockers {
            uid,
            pos: Vector3::new(1450.0f32, 0.0f32, 0.0f32)
        }
    }

    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.pos = pos;
    }
}

impl UIEntity for ScreenBlockers {
    fn get_depth() -> usize {
        0
    }
}

impl Entity for ScreenBlockers {
    fn get_uid(&self) -> UID {
        self.uid
    } 

    fn set_uid(&mut self, uid: UID) {
        self.uid = uid;
    }

    fn get_entity_type(&self) -> EEntityType {
        EEntityType::UIScreenBlockers
    }

    fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
        if render_frame.sqaures.is_none() {
            render_frame.sqaures = Some(vec![]);
        }

        let sqd = SquareRenderData{
            pos: [self.pos.x, self.pos.y],
            height: 2000.0f32,
            width: 2000.0f32,
            color: [0.0f32, 0.0f32, 0.0f32],
            use_border: false
        };

        render_frame.sqaures.as_mut().unwrap().push(sqd);
    }
}