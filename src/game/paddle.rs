use game::entity::{Entity, UID};
use cgmath::Vector3;
use game::ContentId;
use game::world::World;
use graphics::{StaticSprite, SpriteRenderData};

pub struct Paddle {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    uid: UID,
    sprite_id: ContentId,   
}



pub struct PaddleController {

}

impl PaddleController {
    fn update(&self, world: &World) {
        
    }
}

pub struct PaddleModel {

}

impl Paddle {
   
    pub fn new(uid: UID, sprite_id: ContentId) -> Paddle {
        Paddle {
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            uid: uid,
            sprite_id: sprite_id
        }
    }
    
    pub fn update_position(&mut self, delta_time: f32)  {
        self.position.x += 1.0f32 * delta_time
    }

    pub fn set_position(&mut self, pos: Vector3<f32>) {
        self.position = pos;
    }
}

impl Entity for Paddle {
    
    fn get_position(&self) -> Vector3<f32> {
        self.position.clone()
    }


    fn get_scale(&self) -> Vector3<f32>{
        self.scale.clone()
    }

    fn get_rotation(&self) -> Vector3<f32>{
        self.rotation.clone()
    }  

    fn get_uid(&self) -> UID{
        self.uid.clone()
    }
    
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World)>>{
        
        if world.input.is_space_down() {
            
            let move_action = move |world: &mut World| {
                world.pad.set_position(Vector3::new(0.0f32, 0.0, 0.0));
            };

            return Some(Box::new(move_action));        
        }
        
        return None;
    }
}

impl StaticSprite for Paddle{
   fn generate_sprite_render_data(&self) -> SpriteRenderData {
        SpriteRenderData {
            pos: self.position.clone(),
            scale: self.scale.clone(),
            rotation: self.rotation.clone(),
            sprite: self.sprite_id.clone()
        }
    }
}