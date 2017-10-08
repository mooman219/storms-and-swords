use gl;
use gl::types::*;

use glutin;
use glutin::GlContext;

use frame_timer::FrameTimer;
use graphics_new::square_renderer::{SquareRenderData, SquareRenderer};
use graphics_new::circle_renderer::{CircleRenderData, CircleRenderer};


pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer{

        }
    }

    pub fn render(self) {
        let mut frame_timer: FrameTimer = FrameTimer::new();
        let mut events_loop = glutin::EventsLoop::new();

        let window = glutin::WindowBuilder::new().with_title("Storm and Swords").with_dimensions(1200, 800);
        let context = glutin::ContextBuilder::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
        
        let mut srd = SquareRenderData{
            pos:[ 0.0, 0.0],
            height: 1.0,
            width: 1.0,
            color: [0.4,0.5,0.7]
        };

        let mut crd = CircleRenderData {
            pos: [0.0, 0.0],
            height: 1.0, 
            width:1.0,
            color: [0.8, 0.2, 0.7]
        };

        let foo = vec![srd];
        let bar = vec![crd];

        unsafe {
            gl_window.make_current()
        }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _ );

        
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);   
            gl::BindVertexArray(vao);      
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        }

        let square = SquareRenderer::new();
        let mut circle = CircleRenderer::new();

        events_loop.run_forever(|event|{

            frame_timer.frame_start();
            match event {
                glutin::Event::WindowEvent{event, .. } => match event {
                    glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _=>(),
                },
                _ =>()
            }

            unsafe {           
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
             //   Square.render(&foo);
                  circle.render(&bar);
            };

            let _ = gl_window.swap_buffers();

            frame_timer.frame_end();
            glutin::ControlFlow::Continue
        });

            

    }
}