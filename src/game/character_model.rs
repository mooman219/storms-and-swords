use game::*;
use graphics::renderer::RenderFrame;
use cgmath::Vector2;
use graphics::sprite_renderer::SpriteRenderData;

pub struct CharacterModel {
    grid_pos: Vector2<i32>,
    health: i32,
    name: String,
    attack: i32,
    uid: UID,
    reverse: bool
}

impl CharacterModel {
    pub fn new() -> CharacterModel {
        CharacterModel {
            grid_pos: Vector2::new(0, 0),
            health: 0,
            name: String::from("default"),
            attack: 0,
            uid: 0,
            reverse: true
        }
    }

    pub fn from_raw_values(grid_pos: Vector2<i32>, health: i32, name: String, attack: i32, reverse: bool, uid: UID) -> CharacterModel {
        CharacterModel {
            grid_pos,
            health,
            name,
            attack,
            uid,
            reverse
        }
    }

    pub fn set_pos(&mut self, grid_pos: Vector2<i32>) {
        self.grid_pos = grid_pos;
    }

    pub fn get_pos(&self) -> Vector2<i32> {
        self.grid_pos
    }

    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn set_attack(&mut self, attack: i32) {
        self.attack = attack;
    }

    pub fn get_attack(&self) -> i32 {
        self.attack
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    //this clones, be careful of usage
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}


impl Entity for CharacterModel {
    fn get_entity_type(&self) -> EEntityType {
        EEntityType::Character
    }

    fn get_uid(&self) -> UID {
        self.uid
    }

    fn set_uid(&mut self, uid: UID) {
        self.uid = uid;
    }

    fn add_to_render_frame(&self, render_frame: &mut RenderFrame) {
        let render_data = SpriteRenderData {
            pos: [self.grid_pos.x as f32 * 110.0f32 - 990.0, self.grid_pos.y as f32 * 110.0f32],
            sprite_name: String::from("character_sheet_58"),
            height: 110.0f32,
            width: 110.0f32,
            depth: -5.0,
            reverse_x: self.reverse
        };

        if !render_frame.static_sprites.is_some() {
            render_frame.static_sprites = Some(vec![]);
        }
        render_frame.static_sprites.as_mut().unwrap().push(render_data);
    }
}