use game::World;
use graphics::renderer::RenderFrame;


//it is this large for two reasons, one I want to make sure that we never run out to space, and second so that we can have negative uids for flag varibles
pub type UID = u64;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum EEntityType {
    Paddle,
    Ball,
    TetrisBlock,
    UI,
    UIScreenBlockers
}

pub trait Entity {
    fn get_entity_type(&self) -> EEntityType;
    fn get_uid(&self) -> UID;
    fn set_uid(&mut self, uid: UID);
    fn add_to_render_frame(&self, render_frame: &mut RenderFrame);
}

pub trait EntityController {
    fn start(&mut self, world: &mut World);// -> Option<Box<Fn(&mut World, &mut EntityController)>>;
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World, &mut EntityController)>>;
    fn get_entity_type(&self) -> EEntityType;
    fn get_uid(&self) -> UID;   
}
