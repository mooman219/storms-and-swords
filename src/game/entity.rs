use game::World;
use graphics::renderer::RenderFrame;


//0 is a reserved UID, it means invalid, or unset
pub type UID = u64;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum EEntityType {
    UI,
    Character,
    Team,
    Tile,
    BackgroundController,
    GameController,
    Match
}

pub trait Entity {
    fn get_entity_type(&self) -> EEntityType;
    fn get_uid(&self) -> UID;
    fn set_uid(&mut self, uid: UID);
    fn add_to_render_frame(&self, render_frame: &mut RenderFrame);
}

pub trait EntityController {
    fn start(&mut self, world: &mut World);
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World, &mut Box<EntityController>)>>;
    fn get_entity_type(&self) -> EEntityType;
    fn get_uid(&self) -> UID;
}