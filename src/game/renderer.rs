use ::game::component::Component;
use ::math::vector3::Vector3;
use glium::glutin::WindowBuilder;

pub struct Renderer {
    pub model: Vec<Vector3>,
    pub indices: Vec<u32>,
    //TODO: eventaully need a shader programand a uniform
        
}

impl Renderer {

    pub fn Render(&mut self, &mut display, WindowBuilder) -> bool {
        return false;   
    }

}

impl Component for Renderer {

}

