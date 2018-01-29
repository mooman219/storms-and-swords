use game::controller::Controller;
use game::system::*;
use graphics::renderer::RenderFrame;
use cgmath::Vector2;
use graphics::sprite_renderer::SpriteRenderData;

const TILE_WIDTH: f32 = 110.0;
const TILE_HEIGHT: f32 = 110.0;

pub struct GeneratePlayfieldMessage {
}


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
    grid_pos: Vector2<i32>,
    tile_type: ETileType
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            grid_pos: Vector2::new(0, 0),
            tile_type: ETileType::GrassCenter
        }
    }

    pub fn new_with_pos(grid_pos: Vector2<i32>) -> Tile {
        Tile {
            grid_pos,
            tile_type: ETileType::GrassCenter
        }
    }

    pub fn new_with_pos_and_tile_type(grid_pos: Vector2<i32>, tile_type: ETileType) -> Tile {
        Tile {
            grid_pos,
            tile_type
        }
    }

    pub fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
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

pub struct PlayfieldController {
    grid_for_tile: Vec<Vec<Tile>>
}

impl PlayfieldController {
    pub fn new() -> PlayfieldController {
        PlayfieldController {
            grid_for_tile: vec![]
        }
    }

    pub fn new_playfield(&mut self) {
        for x in 0..50 {
            self.grid_for_tile.push(vec![]);
            for y in 0..50 {
                if x >= 1 && x < 18 && y >= 1 && y < 18 {
                    self.grid_for_tile[x].push(
                        Tile::new_with_pos_and_tile_type(
                                Vector2::new(x as i32 * (TILE_WIDTH as i32) - 1000, y as i32 * (TILE_HEIGHT as i32) - 1000),
                                ETileType::GrassCenter
                            )
                        );
                }
                else {
                    self.grid_for_tile[x].push(
                        Tile::new_with_pos_and_tile_type(
                                Vector2::new(x as i32 * (TILE_WIDTH as i32) - 1000, y as i32 * (TILE_HEIGHT as i32) - 1000),
                                ETileType::Ocean
                            )
                    );
                }
            }
        }
    }


    pub fn convert_current_mouse_pos_to_tile(mouse_pos: (f64, f64)) -> (i64, i64){
        let x_tile = ((((mouse_pos.0 * 1000.0) - 500.0) / 55.0) + 0.6).floor() as i64;
        let y_tile = ((((mouse_pos.1 * 1000.0) - 500.0) / 55.0) + 0.6).floor() as i64;
        (x_tile, y_tile)
    }
}

impl Controller for PlayfieldController {
    
    fn start(&mut self) {
        
    }

    fn update(&mut self, message_bag: &mut MessageBag) {
        if message_bag.generate_playfield_messages.len() > 0 {
            message_bag.generate_playfield_messages.drain(..);    
            self.new_playfield();
        }

        println!("{:?}", PlayfieldController::convert_current_mouse_pos_to_tile(message_bag.input.get_current_mouse_pos()));

    }


    fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
        for vec in &self.grid_for_tile {
            for tile in vec {
                tile.add_to_render_frame(render_frame);
            }
        }
    }
}