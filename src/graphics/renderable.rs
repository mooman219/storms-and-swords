use glium::Frame;

pub trait Renderable {
    fn render(&self, frame : &mut Frame);
}