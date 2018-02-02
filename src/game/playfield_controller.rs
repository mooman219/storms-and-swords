use graphics::renderer::{RenderFrame, SCREEN_SCALE, BASE_SCREEN_HEIGHT, BASE_SCREEN_WIDTH};
use cgmath::Vector2;
use game::message_bag::MessageBag;
use graphics::sprite_renderer::SpriteRenderData;
use game::battle_controller::StartBattleMessage;

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

    pub fn set_tile_type (&mut self, tile_type: ETileType) {
        self.tile_type = tile_type;
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
    grid_for_tile: Vec<Vec<Tile>>,
    current_mouse_pos: (f64, f64),
}

impl PlayfieldController {
    pub fn new() -> PlayfieldController {
        PlayfieldController {
            grid_for_tile: vec![],
            current_mouse_pos: (0.0, 0.0)
        }
    }

    pub fn new_playfield(&mut self) {
        for x in 0..50 {
            self.grid_for_tile.push(vec![]);
            for y in 0..50 {
                if x >= 1 && x < 18 && y >= 1 && y < 18 {
                    self.grid_for_tile[x].push(
                        Tile::new_with_pos_and_tile_type(
                                Vector2::new(x as i32 * ((TILE_WIDTH as i32)), y as i32 * (TILE_HEIGHT as i32)),
                                ETileType::GrassCenter
                            )
                        );
                }
                else {
                    self.grid_for_tile[x].push(
                        Tile::new_with_pos_and_tile_type(
                                Vector2::new(x as i32 * ((TILE_WIDTH as i32)), y as i32 * (TILE_HEIGHT as i32)),
                                ETileType::Ocean
                            )
                    );
                }
            }
        }
    }


    pub fn convert_current_mouse_pos_to_tile(mouse_pos: (f64, f64)) -> (usize, usize) {
        let mut x_change = mouse_pos.0 as f32 * (SCREEN_SCALE * BASE_SCREEN_WIDTH) / TILE_WIDTH;
        let mut y_change = mouse_pos.1 as f32 * (SCREEN_SCALE * BASE_SCREEN_HEIGHT) / TILE_HEIGHT;
        if x_change < 0.0 {
            x_change = 0.0; 
        }
        if y_change < 0.0 {
            y_change = 0.0;
        }
        (x_change as usize, y_change as usize)
    }

    pub fn check_for_new_playfield_message(&mut self, message_bag: &mut MessageBag) {
        if message_bag.generate_playfield_messages.len() > 0 {
            message_bag.generate_playfield_messages.drain(..);
            message_bag.start_battle_message.push(StartBattleMessage{});
            self.new_playfield();
        }
    }

    pub fn set_active_tile(&mut self, message_bag: &mut MessageBag) {
        if self.current_mouse_pos != message_bag.input.get_current_mouse_pos() {
            self.current_mouse_pos = message_bag.input.get_current_mouse_pos();
            let grid_index = PlayfieldController::convert_current_mouse_pos_to_tile(self.current_mouse_pos);
            let possible_row = self.grid_for_tile.get_mut(grid_index.0);
            match possible_row {
                Some(row) => {
                    let possible_tile = row.get_mut(grid_index.1);
                    match possible_tile {
                        Some(tile) => {
                            tile.set_tile_type(ETileType::Moutain);               
                        },
                        None => {}}},None => {}
            }
        }
    }

    pub fn render_playfield(&mut self, render_frame: &mut RenderFrame) {
        for vec in &self.grid_for_tile {
                for tile in vec {
                    tile.add_to_render_frame(render_frame);
                }
        }
    }
}