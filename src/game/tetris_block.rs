
#![allow(non_snake_case)]
use glutin;

use game::entity::{Entity, UID, EEntityType, EntityController};
use game::tetris_block_model::TetrisBlockModel;
use cgmath::Vector3;
use game::world::World;
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
                    ((1, 0), false),
                    ((1, 1), true),
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
    pub phantom_cluster: Vec<UID>,
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
            phantom_cluster: vec![],
            frame_count: 0u16,
            tetris_frame: [[None; 10]; 20],
            rng_pool: rand::thread_rng(),
            current_cluster_pos: vec![],
            spawn_pos: (5, 0)
        }
    }

    //this assumes that new_pos is a set of valid(open) indexes in the tetris_Frame
    //it will then move the current cluster to that positions, so this is used by move, rotate, and drop
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
  
    //this will create the blocks, the indexs, and accounting for a new tetris cluster
    //at the end of this function, the new indexs in the tetris_frame should be set with the new uids
    //current_cluster_pos should contain all those indexs
    //and the entity should be held by world
    pub fn genererate_and_place_next_tetris_block(&mut self, inner_world: &mut World){

        let color_v = Range::new(0.25f32, 0.75f32);
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
            let pos = Vector3::new(100.0f32 * (block_pos.0 as f32 + 1f32), 950f32 + (100f32 * -(block_pos.1 as f32)), 250f32);
            let tbm = TetrisBlockModel::new(pos, color, 0u64, marked);
            let new_uid = inner_world.set_uid_for_entity(Box::new(tbm));
            self.tetris_frame[(self.spawn_pos.1 + block_pos.1) as usize][(self.spawn_pos.0 + block_pos.0) as usize + 1] = Some(new_uid);
            self.current_cluster_pos.push((self.spawn_pos.0 + block_pos.0 + 1, self.spawn_pos.1 + block_pos.1));
        }
    }

    //called once at the start of the game, this creates the phantom blocks that are used for the phantom cluster
    //which is used for showing where the players tetris piece would end if they droped it now
    pub fn create_phantom_cluster(&mut self, inner_world: &mut World) {
        for _ in 0..4 {
            let tbm = TetrisBlockModel::new(Vector3::new(0.0f32, 0.0f32, 0.0f32), [1.0f32, 0.0f32, 0.0f32, 0.5f32], 0u64, false);
            self.phantom_cluster.push(inner_world.set_uid_for_entity(Box::new(tbm)));
        }
    }

    //this will genreate the next down positions for each block in the cluster
    //returning none if any of the new places are not valid
    //you must always provide the current cluster positions
    pub fn generate_next_down_position_for_cluster(cluster: &Vec<Vec2>) -> Option<Vec<Vec2>> {
        
        let mut new_pos : Vec<Vec2> = vec![];

        for pos in cluster {
            let new_y = pos.1 + 1;
            if new_y < 0 || new_y > 19{
                return None;
            }

            new_pos.push((pos.0, new_y));
        }
        return Some(new_pos);
    }

    //a negative direction indicates a move to the left, and positive direction indicates a move to the right
    pub fn genertate_next_move_for_cluster(direction: i8, cluster: &Vec<Vec2>) -> Option<Vec<Vec2>>{
       let mut new_pos : Vec<Vec2> = vec![];

       for pos in cluster {
           let new_x = pos.0 + direction;
           if new_x < 0 || new_x > 9 {
               return None;
           }
           new_pos.push((new_x, pos.1));
       }
       return Some(new_pos); 
    }

    //will rotate the blocks by 90 degrees, around the "marked" or "middle" block of the cluster
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
        

        //we can rotate a point by simply switching the x and y, and then negating one based on the direction we want to go in
        //(1,0) is a 90 degree rotation from (0, 1), and (0, -1)
        for pos in &self.current_cluster_pos {

            let mut new_x = pos.0 - marked_pos.0;
            let mut new_y = pos.1 - marked_pos.1;

            if direction == 1 {
                new_x = new_x * -1;
            }
            else if direction == -1 {
                new_y = new_y * -1;
            }
            let temp = new_x;
            let new_x = new_y;
            let new_y = temp;

            
            new_pos.push((new_x as i8 + marked_pos.0, new_y as i8 + marked_pos.1));
        }
        for pos in &new_pos {
            if pos.0 < 0 || pos.0 > 9 {
                return None;
            }

            if pos.1 < 0 || pos.1 > 19 {
                return None;
            }
        }

        return Some(new_pos);
    }

    pub fn phantom_cluster_placement(&mut self, inner_world: &mut World) {
        if self.current_cluster_pos.len() > 0 {
            let pos = self.generate_drop_block_indexes();

            for i in 0..self.phantom_cluster.len() {
                let phantom = inner_world.get_mut_entity(self.phantom_cluster[i]);
                match phantom {
                    Some(phantom) => {
                        let phantom = unsafe { &mut *(phantom as *mut &Entity as *mut &mut TetrisBlockModel) };
                        phantom.pos = Vector3::new((pos[i].0 as i8 - 5) as f32 * 100f32 , 950f32 + (100f32 * -(pos[i].1 as f32)), 250f32);
                    },
                    None => {

                    }
                }
            }
        }
    }

    pub fn handle_block_drop_and_quick_down(&mut self, inner_world: &mut World) {
        if inner_world.get_input().on_key_released(glutin::VirtualKeyCode::Space) {
            let drop_pos = self.generate_drop_block_indexes();
            self.update_current_cluster(inner_world, &drop_pos);
            self.finish_with_current_cluster(inner_world);
            self.frame_count = 0u16;
        }
            

        if inner_world.get_input().on_key_held(glutin::VirtualKeyCode::Down) {
            self.frame_count += 12;
        }
    }

    pub fn handle_rotation(&mut self, inner_world: &mut World) {
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
            let lateral_move_attempt = TetrisBlockController::genertate_next_move_for_cluster(direction, &self.current_cluster_pos.clone());
            if lateral_move_attempt.is_some() {
                if self.are_unoccupied_and_not_me(&lateral_move_attempt.as_ref().unwrap()) {
                    self.update_current_cluster(inner_world, &lateral_move_attempt.unwrap());
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
        else if inner_world.get_input().on_key_pressed(glutin::VirtualKeyCode::Up) {
            rotate_direction = -1;
        }

        if rotate_direction != 0 {
            let rotate_move = self.generate_next_rotation_for_cluster(inner_world, rotate_direction);
            if rotate_move.is_some() {
                if self.are_unoccupied_and_not_me(&rotate_move.as_ref().unwrap()) {
                    self.update_current_cluster(inner_world, &rotate_move.unwrap());
                }
            }
        }
    }

    //helper function that will likely have more to do 
    pub fn finish_with_current_cluster(&mut self, inner_world: &mut World) {
        self.current_cluster_pos.truncate(0);

        let lines = self.look_for_solid_lines();
        if lines.len() != 0 {
            for index in lines {
                let len = self.tetris_frame[index].len();
                for i in 0..len {
                    let uid = self.tetris_frame[index][i].unwrap();
                    inner_world.delete_entity(uid);
                    self.tetris_frame[index][i] = None;
                }
            }
        
            let mut new_frame : [[Option<UID>; 10]; 20] = [[None; 10]; 20];
            let mut fake_y = 0;
            for y in 0..20 {
                if TetrisBlockController::is_line_empty(&self.tetris_frame[19 - y]) {
                        continue;
                }
                else {
                    for x in 0..10 {
                        new_frame[19 - fake_y][x] = self.tetris_frame[19 - y][x];
                    }
                    fake_y = fake_y + 1;
                }
            }
            self.tetris_frame = new_frame;
            for y in 0..20 {
                for x in 0..10 {
                    let uid_maybe = self.tetris_frame[y][x];
                    match uid_maybe {
                        Some(uid) => {
                            let tetris_piece = inner_world.get_mut_entity(uid);
                            match tetris_piece {
                                Some(tetris_piece) => {
                                    let tetris_piece = unsafe{&mut *(tetris_piece as  *mut &Entity as *mut &mut TetrisBlockModel)};
                                    tetris_piece.pos = Vector3::new((x as i8 - 5) as f32 * 100f32 , 950f32 + (100f32 * -(y as f32)), 250f32);
                                },
                                None => {

                                }
                            }
                        },
                        None => {

                        }
                    }
                }
            }
        }
    }
    pub fn is_line_empty(line: &[Option<UID>; 10]) -> bool {
        for el in line {
            if el.is_some() {
                return false;
            }
        }
        true
    }

    //this will test to make sure that the positions you want to move to are both, open, if not, are not the current cluster itself
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

    //this will always return a value, because at the very least a cluster can stay in place and that be a valid drop

    pub fn generate_drop_block_indexes(&self) -> Vec<Vec2>{
        let mut final_pos = Some(self.current_cluster_pos.clone());
        
        while final_pos.as_ref().is_some() {
            let check = TetrisBlockController::generate_next_down_position_for_cluster(final_pos.as_ref().unwrap());
            if check.is_some() {
                if self.are_unoccupied_and_not_me(&check.as_ref().unwrap()) {
                    final_pos = check;
                }
                else {
                    return final_pos.unwrap();
                }
            }
            else {
                return final_pos.unwrap();
            }
        }

        final_pos.unwrap()
    }

    //this will return y index of all rules that are full in the current frame
    //it does not clear them
    pub fn look_for_solid_lines(&self) -> Vec<usize> {
 
        let mut indexes = vec![];

        for index in 0..20 {
            let mut count = 0;
            for x_index in 0..10 {
                let val =  self.tetris_frame[index][x_index];
                    
                if val != None {
                    count = count + 1;
                }
            }
            if count == 10 {
                indexes.push(index);
            }
        }
        indexes
    }
}

impl EntityController for TetrisBlockController {

    fn get_uid(&self) -> UID {
        self.uid
    }

    fn start(&mut self, world: &mut World) {
        
        self.create_phantom_cluster(world);
        
    }
    
    //called once pre frame, it updates the current cluster, and handle the condition of a soild line forming
    fn update(&self, _world: &World) ->  Option<Box<Fn(&mut World, &mut EntityController)>> {

         let return_closure = move |inner_world: &mut World, controller: &mut EntityController| {

            let tbc =  unsafe { &mut *(controller as *mut EntityController as *mut TetrisBlockController) };

            if tbc.current_cluster_pos.len() == 0 {
                //if we have no current cluster, create and place a new one
                tbc.genererate_and_place_next_tetris_block(inner_world);
            }
            else {
                tbc.frame_count = tbc.frame_count + 1u16;

                tbc.handle_rotation(inner_world);

                tbc.handle_block_drop_and_quick_down(inner_world);

                tbc.phantom_cluster_placement(inner_world);
                
                if tbc.frame_count >= 45u16 {

                    let next_pos;
                    next_pos = TetrisBlockController::generate_next_down_position_for_cluster(&tbc.current_cluster_pos.clone());
 
                    match next_pos {
                        Some(next_pos) => {
                            
                            let are_clear = tbc.are_unoccupied_and_not_me(&next_pos);
                            
                            if are_clear {
                                tbc.update_current_cluster(inner_world, &next_pos);
                                
                            }
                            else {
                                tbc.finish_with_current_cluster(inner_world);
                            }
                        }
                        None => {
                            //in this case we can assume that a generated position is out of bounds, so we just stop the movmenet
                            tbc.finish_with_current_cluster(inner_world);
                        }
                    }
                    tbc.frame_count = 0u16;
                }
            }
        };

        return Some(Box::new(return_closure));
    }

    fn get_entity_type(&self) -> EEntityType {
        EEntityType::TetrisBlock
    }
}