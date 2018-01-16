use game::*;
use cgmath::Vector2;
use graphics::renderer::RenderFrame;
use graphics::sprite_renderer::SpriteRenderData;

const TILE_WIDTH: f32 = 110.0;
const TILE_HEIGHT: f32 = 110.0;


pub enum ETileType {
    GrassCenter,
    RiverUp,
    Ocean,
    Moutain,
    Trees
}

impl ETileType {
    pub fn to_sprite_name(&self) -> String {
        match *self {
            ETileType::GrassCenter => {
                String::from("tileset_15")
            },
            ETileType::Moutain => {
                String::from("tileset_1")
            },
            ETileType::Ocean => {
                String::from("tileset_151")
            },
            ETileType::RiverUp => {
                String::from("tileset_435")
            },
            ETileType::Trees => {
                String::from("tileset_61")
            }
        }
    }
}

pub struct Tile {
    uid: UID,
    grid_pos: Vector2<i32>,
    tile_type: ETileType
}

impl Tile {
    pub fn new(uid: UID) -> Tile {
        Tile {
            uid,
            grid_pos: Vector2::new(0, 0),
            tile_type: ETileType::GrassCenter
        }
    }

    pub fn new_with_pos(uid: UID, grid_pos: Vector2<i32>) -> Tile {
        Tile {
            uid,
            grid_pos,
            tile_type: ETileType::GrassCenter
        }
    }

pub fn new_with_pos_and_tile_type(uid: UID, grid_pos: Vector2<i32>, tile_type: ETileType) -> Tile {
        Tile {
            uid,
            grid_pos,
            tile_type
        }
    }
}

impl Entity for Tile {
    fn get_entity_type(&self) -> EEntityType{
        EEntityType::Tile
    }

    fn get_uid(&self) -> UID{
        self.uid
    }

    fn set_uid(&mut self, uid: UID){
        self.uid = uid;
    }

    fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
        let render_data = SpriteRenderData {
            pos: [self.grid_pos.x as f32, self.grid_pos.y as f32],
            sprite_name: self.tile_type.to_sprite_name(),
            height: TILE_HEIGHT,
            width: TILE_WIDTH,
            depth: -6.0,
            reverse_x: false
        };

        if !render_frame.static_sprites.is_some() {
            render_frame.static_sprites = Some(vec![]);
        }
        render_frame.static_sprites.as_mut().unwrap().push(render_data);
    }
}

pub struct BackgroundController {
    uid: UID,
    grid_for_tiles: Vec<Vec<UID>>,
    count: i32
}

impl BackgroundController {
   
    pub fn new(uid: UID, world: &mut World) -> BackgroundController {
        let mut tiles = vec![];
        
        for x in 0..50 {
            tiles.push(vec![]);
            for y in 0..50 {

//                let dis = ()
                if x >= 1 && x < 18 && y >= 1 && y < 18 {
                    tiles[x].push(world.set_uid_for_entity(Box::new(Tile::new_with_pos_and_tile_type(
                        0,
                        Vector2::new(x as i32 * (TILE_WIDTH as i32) - 990, y as i32 * (TILE_HEIGHT as i32) - 1000),
                        ETileType::GrassCenter
                    ))));
                }
                else {
                    tiles[x].push(world.set_uid_for_entity(Box::new(Tile::new_with_pos_and_tile_type(
                        0,
                        Vector2::new(x as i32 * (TILE_WIDTH as i32) - 990, y as i32 * (TILE_HEIGHT as i32) - 1000),
                        ETileType::Ocean
                    ))));
                }
            }
        }

        BackgroundController {
            uid,
            grid_for_tiles: tiles,
            count: 0
        }

    }
}

impl EntityController for BackgroundController {

    fn start(&mut self, world: &mut World){

    }

    fn update(&self, world: &World) -> Option<Box<Fn(&mut World, &mut Box<EntityController>)>>{
        None
    }
    
    fn get_entity_type(&self) -> EEntityType{
        EEntityType::BackgroundController
    }
    
    fn get_uid(&self) -> UID{
        self.uid
    }
}