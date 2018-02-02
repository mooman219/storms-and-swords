use game::system::*;
use game::message_bag::MessageBag;
use graphics::renderer::RenderFrame;

pub trait Controller {
    fn start(&mut self);
    fn update(&mut self, message_bag: &mut MessageBag);
    fn add_to_render_frame(&self, render_frame: &mut RenderFrame);
}