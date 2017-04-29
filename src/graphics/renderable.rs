use glium::Frame;
use game::entity::Entity;

pub trait Renderable {
    fn render(&self, entity: &Entity, frame: Frame);
}
