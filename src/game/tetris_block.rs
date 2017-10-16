use game::entity::{Entity, UID, EEntityType, EntityController};
use cgmath::Vector3;
use game::world::World;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use rand::Rng;
use rand;
use rand::distributions::{IndependentSample, Range};

pub enum TetrominoType {
    Line,
    Sqaure,
    Tee,
    El,
    S,
    Z
}

pub struct TetrisBlockController {
    pub uid: UID,
    pub current_cluster: Vec<UID>,
    pub frame_count: u16,
}

impl<'a>TetrisBlockController {

    pub fn new(uid: UID) -> TetrisBlockController {
        TetrisBlockController {
            uid,
            current_cluster: vec![],
            frame_count: 0u16
        }
    }

    pub fn create_random_cluster(&mut self) {
         
    }


}

impl<'a> EntityController for TetrisBlockController {

    fn get_uid(&self) -> UID {
        self.uid
    }
    
    fn update(&self, _world: &World) ->  Option<Box<Fn(&mut World, &mut EntityController)>> {

         let return_closure = move |inner_world: &mut World, controller: &mut EntityController| {
            let color_v = Range::new(0.0f32, 1.0f32);
            let tbc =  unsafe { &mut *(controller as *mut EntityController as *mut TetrisBlockController) };
            if tbc.current_cluster.len() == 0 {
                let mut rng = rand::thread_rng();
                for i in 0..10 {
                    let color : [f32; 4] = [color_v.ind_sample(&mut rng), color_v.ind_sample(&mut rng), color_v.ind_sample(&mut rng), 255.0f32];
                    let tbm = TetrisBlockModel::new(Vector3::new((100.0f32 * (i as f32)) - 450.0f32, 550.0f32, 250.0f32), color, 0u64);
                    tbc.current_cluster.push(inner_world.set_uid_for_entity(Box::new(tbm)));
                }
            }
            else {
                tbc.frame_count = tbc.frame_count + 1u16;
                let mut mark_for_drop = false;
                if tbc.frame_count == 60u16 {
                    for uid in tbc.current_cluster.iter() {
                        let mut tetris_piece = inner_world.get_mut_entity(*uid).unwrap();
                        let mut tetris_piece = unsafe {&mut *(tetris_piece as *mut &Entity as *mut  &mut TetrisBlockModel)};
                        tetris_piece.pos = Vector3::new(tetris_piece.pos.x, tetris_piece.pos.y - 100f32, tetris_piece.pos.z);
                        if tetris_piece.pos.y == -650.0f32 {
                            mark_for_drop = true;
                        }
                    }
                    tbc.frame_count = 016;

                }

                if mark_for_drop {
                    tbc.current_cluster.truncate(0);
                }
            }
        };

        return Some(Box::new(return_closure));
    }

    fn get_entity_type(&self) -> EEntityType {
        EEntityType::TetrisBlock
    }


}

#[derive(Copy, Clone)]
pub struct TetrisBlockModel {
    pub pos: Vector3<f32>,
    pub color: [f32;4],
    pub uid: UID,
    
}

impl TetrisBlockModel {
    pub fn new(pos: Vector3<f32>, color: [f32;4], uid: UID) -> TetrisBlockModel {
        TetrisBlockModel {
            pos,
            color,
            uid
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
            color: [self.color[0], self.color[1], self.color[2]]
        };

        render_frame.sqaures.as_mut().unwrap().push(sqd);
    }
} 