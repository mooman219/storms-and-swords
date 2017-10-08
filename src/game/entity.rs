use game::World;
use graphics::renderer::RenderFrame;


//it is this large for two reasons, one I want to make sure that we never run out to space, and second so that we can have negative uids for flag varibles
pub type UID = u64;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum EEntityType {
    PADDLE,
    BALL,
}

pub trait Entity {
  fn get_entity_type(&self) -> EEntityType;
  fn get_uid(&self) -> UID;
  fn add_to_render_frame(&self, render_frame: &mut RenderFrame);
}

pub trait EntityController {
    fn update(&self, world: &World) -> Option<Box<Fn(&mut World)>>;
    fn get_entity_type(&self) -> EEntityType;
}