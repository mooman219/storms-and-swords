
#![allow(non_snake_case)]


use game::entity::{Entity, UID, EEntityType, EntityController};
use cgmath::Vector3;
use game::world::World;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use rand;
use rand::distributions::{IndependentSample, Range};

type Vec2 = (i8, i8);

#[derive(Copy, Clone)]
pub enum TetrominoType {
    Line,
    Sqaure,
    Tee,
    El,
    S,
    Z
}

impl TetrominoType {
    pub fn offsets(&self) -> Vec<Vec2> {
        match *self {
            TetrominoType::Line => {
                return vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3)
                ];
            },
            TetrominoType::Sqaure => {
                return vec![
                    (0, 0),
                    (1, 0),
                    (0, 1),
                    (1, 1)
                ];
            },
            TetrominoType::Tee => {
                return vec![
                    (0, 0),
                    (1, 0),
                    (-1, 0),
                    (0, 1)
                ];
            },
            TetrominoType::El => {
                return vec![
                    (0, 0),
                    (1, 0),
                    (1, 1),
                    (1, 2)
                ];
            },
            TetrominoType::S => {
                return vec![
                    (0, 0),
                    (-1, 0),
                    (0, 1),
                    (1, 1)
                ];                
            },
            TetrominoType::Z => {
                return vec![
                    (0, 0),
                    (1, 0),
                    (0, 1),
                    (-1, 1)
                ];                
            }
        }
    }

    pub fn from_usize(index: usize) -> Option<TetrominoType> {
        
        if index >= 6 {
            return None;
        }
        else {
            match index {
                0 => {
                    Some(TetrominoType::Line)
                },
                1 => {
                    Some(TetrominoType::Sqaure)
                },
                2 => {
                    Some(TetrominoType::Tee)
                },
                3 => {
                    Some(TetrominoType::El)
                },
                4 => {
                    Some(TetrominoType::S)
                },
                5 => {
                    Some(TetrominoType::Z)
                },
                _ => {
                    None
                }
            }
        }
    }
}

pub struct TetrisBlockController {
    pub uid: UID,
    pub current_cluster: Vec<UID>,
    pub frame_count: u16,
    pub tetris_frame: [[Option<UID>; 10]; 20],
    pub rng_pool: rand::ThreadRng,
    pub current_cluster_pos: Vec<Vec2>,
    pub spawn_pos: Vec2
}

impl<'a>TetrisBlockController {

    pub fn new(uid: UID) -> TetrisBlockController {
        TetrisBlockController {
            uid,
            current_cluster: vec![],
            frame_count: 0u16,
            tetris_frame: [[None; 10]; 20],
            rng_pool: rand::thread_rng(),
            current_cluster_pos: vec![],
            spawn_pos: (5, 0)
        }
    }
  
    pub fn genererate_and_place_next_tetris_block(&mut self, inner_world: &mut World){

        let color_v = Range::new(0.0f32, 1.0f32);
        let index = Range::new(0usize, 6usize);
        let index = index.ind_sample(&mut self.rng_pool);
        let offsets = TetrominoType::from_usize(index).unwrap().offsets();

        let color : [f32; 4] = [color_v.ind_sample(&mut self.rng_pool),//r 
                                color_v.ind_sample(&mut self.rng_pool),//g
                                color_v.ind_sample(&mut self.rng_pool), //b
                                255.0f32];//a

        for offset in offsets {
            let pos = Vector3::new(100.0f32 * (offset.0 as f32), 550f32 - (100f32 * -offset.1 as f32), 250f32);
            let tbm = TetrisBlockModel::new(pos, color, 0u64);
            let new_uid = inner_world.set_uid_for_entity(Box::new(tbm));
            self.tetris_frame[(self.spawn_pos.1 + offset.1) as usize][(self.spawn_pos.0 + offset.0) as usize] = Some(new_uid);
            self.current_cluster_pos.push((self.spawn_pos.0 + offset.0, self.spawn_pos.1 + offset.1));
        }
  }

    pub fn generate_next_positions_for_cluster(&self) -> Option<Vec<Vec2>> {
        let mut new_pos : Vec<Vec2> = vec![];
        for pos in &self.current_cluster_pos {
            let new_y = pos.1 + 1;
            if new_y < 0 || new_y > 19{
                return None;
            }

            new_pos.push((pos.0, new_y));
        }
        return Some(new_pos);
    }

    pub fn finish_with_current_cluster(&mut self) {
        self.current_cluster_pos.truncate(0);
    }

    pub fn are_unoccupied_and_not_me(&self, posistions: Vec<Vec2>) -> bool {

        for pos in posistions {
            if self.tetris_frame[pos.1 as usize][pos.0 as usize] != None {
                if !self.current_cluster_pos.contains(&pos) {
                    return false;
                }
            }
        }

        return true;
    }

}

impl EntityController for TetrisBlockController {

    fn get_uid(&self) -> UID {
        self.uid
    }
    
    fn update(&self, _world: &World) ->  Option<Box<Fn(&mut World, &mut EntityController)>> {

         let return_closure = move |inner_world: &mut World, controller: &mut EntityController| {

            let tbc =  unsafe { &mut *(controller as *mut EntityController as *mut TetrisBlockController) };

            if tbc.current_cluster_pos.len() == 0 {
                //if we have no current cluster, create and place a new one
                tbc.genererate_and_place_next_tetris_block(inner_world);
            }
            else {
                
                tbc.frame_count = tbc.frame_count + 1u16;

                if tbc.frame_count == 60u16 {

                    let next_pos = tbc.generate_next_positions_for_cluster();

                    match next_pos {
                        Some(next_pos) => {

                            let are_clear = tbc.are_unoccupied_and_not_me(next_pos);
                            

                            if are_clear {



                                let old_poses = tbc.current_cluster_pos.clone();
                                tbc.current_cluster_pos.truncate(0);
                                
                                let old_frame = tbc.tetris_frame.clone();
                                let mut use_uids = vec![];

                                for pos in &old_poses {
                                    let uid = old_frame[pos.1 as usize][pos.0 as usize].unwrap();
                                    tbc.tetris_frame[pos.1 as usize][pos.0 as usize] = None;
                                    use_uids.push((uid, pos));
                                }

                                for k in use_uids.iter() {
                                    let tetris_piece = inner_world.get_mut_entity(k.0).unwrap();
                                    let tetris_piece = unsafe {&mut *(tetris_piece as *mut &Entity as *mut  &mut TetrisBlockModel)};
                                    tetris_piece.pos = Vector3::new(tetris_piece.pos.x, tetris_piece.pos.y - 100f32, tetris_piece.pos.z);
                      //              tbc.tetris_frame[(k.1).1 as usize][(k.1).0 as usize] = None;
                                    tbc.tetris_frame[(k.1).1 as usize + 1][(k.1).0 as usize] = Some(k.0);
                                    tbc.current_cluster_pos.push(((k.1).0, (k.1).1 + 1));
                                }



                            }
                            else {
                                tbc.finish_with_current_cluster();
                            }

                            tbc.frame_count = 016;
                        }
                        None => {
                            //in this case we can assume that a generated position is out of bounds, so we just stop the movmenet
                            tbc.finish_with_current_cluster();
                        }
                    }

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