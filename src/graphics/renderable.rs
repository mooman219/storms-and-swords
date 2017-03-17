use glium::Frame;

pub trait Renderable {
    fn render(&mut self, frame : &mut Frame);
}