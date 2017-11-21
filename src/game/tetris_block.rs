
#![allow(non_snake_case)]
use glutin;

use game::entity::{Entity, UID, EEntityType, EntityController};
use cgmath::Vector3;
use game::world::World;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::f32;

type Vec2 = (i8, i8);
type Vec2PlusMarked = (Vec2, bool);

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
    pub fn offsets(&self) -> Vec<Vec2PlusMarked> {
        match *self {
            TetrominoType::Line => {
                return vec![
                    ((0, 0), false),
                    ((0, 1), true),
                    ((0, 2), false),
                    ((0, 3), false)
                ];
            },
            TetrominoType::Sqaure => {
                return vec![
                    ((0, 0), false),
                    ((1, 0), false),
                    ((0, 1), false),
                    ((1, 1), false)
                ];
            },
            TetrominoType::Tee => {
                return vec![
                    ((0, 0), true),
                    ((1, 0), false),
                    ((-1, 0), false),
                    ((0, 1), false)
                ];
            },
            TetrominoType::El => {
                return vec![
                    ((0, 0), false),
                    ((1, 0), true),
                    ((1, 1), false),
                    ((1, 2), false)
                ];
            },
            TetrominoType::S => {
                return vec![
                    ((0, 0), true),
                    ((-1, 0), false),
                    ((0, 1), false),
                    ((1, 1), false)
                ];                
            },
            TetrominoType::Z => {
                return vec![
                    ((0, 0), true),
                    ((1, 0), false),
                    ((0, 1), false),
                    ((-1, 1), false)
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

    pub fn update_current_cluster(&mut self, inner_world: &mut World, new_pos: &Vec<Vec2>) {
        let old_poses = self.current_cluster_pos.clone();
        self.current_cluster_pos.truncate(0);

        let old_frame = self.tetris_frame.clone();
        let mut use_uids = vec![];

        for pos in &old_poses {
            let uid = old_frame[pos.1 as usize][pos.0 as usize].unwrap();
            self.tetris_frame[pos.1 as usize][pos.0 as usize] = None;
            use_uids.push((uid, pos));
        }
        let mut tet_pies = vec![];

        for uids_and_pos in use_uids.iter() {
          let tetris_piece = inner_world.get_mut_entity(uids_and_pos.0).unwrap();
          
          let tetris_piece = unsafe{&mut *(tetris_piece as *mut &Entity as *mut &mut TetrisBlockModel)};
          tet_pies.push(tetris_piece);
        }

        let mut count = 0;
        for pos in new_pos.iter() {
            tet_pies.get_mut(count).as_mut().unwrap().pos = Vector3::new((pos.0 - 5) as f32 * 100f32 , 950f32 + (100f32 * -(pos.1 as f32)), 250f32);
            self.tetris_frame[pos.1 as usize][pos.0 as usize] = Some(tet_pies.get_mut(count).unwrap().get_uid());
            count = count + 1;
        }

        self.current_cluster_pos = new_pos.clone();
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
            let block_pos = offset.0;
            let marked = offset.1;
            let pos = Vector3::new(100.0f32 * (block_pos.0 as f32), 950f32 + (100f32 * -(block_pos.1 as f32)), 250f32);
            let tbm = TetrisBlockModel::new(pos, color, 0u64, marked);
            let new_uid = inner_world.set_uid_for_entity(Box::new(tbm));
            self.tetris_frame[(self.spawn_pos.1 + block_pos.1) as usize][(self.spawn_pos.0 + block_pos.0) as usize] = Some(new_uid);
            self.current_cluster_pos.push((self.spawn_pos.0 + block_pos.0, self.spawn_pos.1 + block_pos.1));
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

    //a negative direction indicates a move to the left, and positive direction indicates a move to the right
    pub fn genertate_next_move_for_cluster(&self, direction: i8) -> Option<Vec<Vec2>>{
       let mut new_pos : Vec<Vec2> = vec![];

       for pos in &self.current_cluster_pos {
           let new_x = pos.0 + direction;
           if new_x < 0 || new_x > 9 {
               return None;
           }
           new_pos.push((new_x, pos.1));
       }
       return Some(new_pos); 
    }

    pub fn generate_next_rotation_for_cluster(&self, inner_world: &mut World, direction: i8) -> Option<Vec<Vec2>> {
     
        let mut is_marked = false;
        let mut marked_pos = (0i8, 0i8);
        for pos in &self.current_cluster_pos {

            let uid = self.tetris_frame[pos.1 as usize][pos.0 as usize];

            let tetris_piece = inner_world.get_mut_entity(uid.unwrap()).unwrap();
          
            let tetris_piece = unsafe{&mut *(tetris_piece as  *mut &Entity as *mut &mut TetrisBlockModel)};
            if tetris_piece.is_middle {
                is_marked = true;
                marked_pos = *pos;
            }
        }

        if !is_marked {
            //there is no middle block for the sqaure tetrinmo, so just return the current positions
            return Some(self.current_cluster_pos.clone());
        }

        let mut new_pos: Vec<Vec2> = vec![];
        for pos in &self.current_cluster_pos {
            let new_x = pos.0 as f32 * f32::cos((f32::consts::PI / 2.0f32 * direction as f32) as f32) - 
                        pos.1 as f32 * f32::sin((f32::consts::PI / 2.0f32 * direction as f32) as f32);

            let new_y = pos.1 as f32 * f32::cos((f32::consts::PI / 2.0f32 * direction as f32) as f32) - 
                        pos.0 as f32 * f32::sin((f32::consts::PI / 2.0f32 * direction as f32) as f32); 
            
            new_pos.push((new_x as i8 + marked_pos.0, new_y as i8 + marked_pos.1));
        }

        return Some(new_pos);
    }

    pub fn finish_with_current_cluster(&mut self) {
        self.current_cluster_pos.truncate(0);
    }


    pub fn are_unoccupied_and_not_me(&self, posistions: &Vec<Vec2>) -> bool {

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


                //we first check to see if they want to move, and move them, and then do the same for rotate
                let mut direction = 0;
                if inner_world.get_input().on_key_pressed(glutin::VirtualKeyCode::Left) {
                    //generate, test, and respond to a left move
                    direction = -1;
                }
                else if inner_world.get_input().on_key_pressed(glutin::VirtualKeyCode::Right) {
                    //generate, test, and respond to a right move
                    direction = 1;
                }

                if direction != 0 {
                    let lateral_move_attempt = tbc.genertate_next_move_for_cluster(direction);
                    if lateral_move_attempt.is_some() {

                        if tbc.are_unoccupied_and_not_me(&lateral_move_attempt.as_ref().unwrap()) {
                            tbc.update_current_cluster(inner_world, &lateral_move_attempt.unwrap());
                        }
                    }
                }


                let mut rotate_direction = 0;
                if inner_world.get_input().on_key_pressed(glutin::VirtualKeyCode::E) {
                    rotate_direction = 1;
                }
                else if inner_world.get_input().on_key_pressed(glutin::VirtualKeyCode::Q) {
                    rotate_direction = -1;
                }

                if rotate_direction != 0 {
                    println!("Hello");
                    //this takes in inner world because we have to look up which of the tetris blocks is seen as the middle
                    let rotate_move = tbc.generate_next_rotation_for_cluster(inner_world, rotate_direction);
                    println!("{:?}", tbc.current_cluster_pos);
                    println!("{:?}", rotate_move);
                    if rotate_move.is_some() {
                        if tbc.are_unoccupied_and_not_me(&rotate_move.as_ref().unwrap()) {
                            tbc.update_current_cluster(inner_world, &rotate_move.unwrap());
                        }
                    }
                }

                

                if tbc.frame_count == 60u16 {

                    let next_pos;
                    next_pos = tbc.generate_next_positions_for_cluster();
 
                    match next_pos {
                        Some(next_pos) => {
                            
                            let are_clear = tbc.are_unoccupied_and_not_me(&next_pos);
                            
                            if are_clear {
                                tbc.update_current_cluster(inner_world, &next_pos);
                                
                            }
                            else {
                                tbc.finish_with_current_cluster();
                            }
                        }
                        None => {
                            //in this case we can assume that a generated position is out of bounds, so we just stop the movmenet
                            tbc.finish_with_current_cluster();
                        }
                    }
                    tbc.frame_count = 016;
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
            color: [self.color[0], self.color[1], self.color[2]]
        };

        render_frame.sqaures.as_mut().unwrap().push(sqd);
    }
} 