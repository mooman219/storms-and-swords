use gl;
use gl::types::*;

use cgmath::{Matrix4, ortho};
use std::collections::HashMap;

use std::sync::mpsc::{Receiver, Sender};

use glutin;
use glutin::{GlContext, VirtualKeyCode};
use content::load_content::{EContentType, EContentLoadRequst};

use frame_timer::FrameTimer;
use graphics::square_renderer::{SquareRenderData, SquareRenderer};
use graphics::circle_renderer::{CircleRenderData, CircleRenderer};
use graphics::frame_renderer::{FrameRenderer};


#[derive(Clone)]
pub struct RenderFrame {
    pub frame_index: u64,
    pub sqaures: Option<Vec<SquareRenderData>>,
    pub circles: Option<Vec<CircleRenderData>>,
}

impl RenderFrame {
    pub fn new(frame_index: u64, sqaures: Option<Vec<SquareRenderData>>, circles: Option<Vec<CircleRenderData>>) -> RenderFrame {
        RenderFrame {
            frame_index: frame_index,
            sqaures: sqaures,
            circles: circles,
        }
    }
}

pub struct Renderer {
    pub ortho_matrix: Matrix4<GLfloat>,
    from_game_thread: Receiver<RenderFrame>,
    _to_content_manifest: Sender<EContentLoadRequst>,
    _from_content_manifest: Receiver<EContentType>,
    to_game_thread_with_input: Sender<glutin::KeyboardInput>,
    _sprite_name_to_texture_id: HashMap<String, GLuint>,
}

impl Renderer {
    pub fn new(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<glutin::KeyboardInput>,
    ) -> Renderer {
        Renderer {
            ortho_matrix: ortho(-1000.0f32, 1000.0f32, -1000.0f32, 1000.0f32, 0.0, 10.0),
            from_game_thread: from_game_thread,
            _to_content_manifest: to_content_manifest,
            _from_content_manifest: from_content_manifest,
            to_game_thread_with_input: to_game_thread_with_input,
            _sprite_name_to_texture_id: HashMap::new(),
        }
    }

    pub fn render_thread(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<glutin::KeyboardInput>,
    ) {


        let render_thread = Renderer::new(
            from_game_thread,
            to_content_manifest,
            from_content_manifest,
            to_game_thread_with_input,
        );
        render_thread.render();
    }

    pub fn render(self) {

        let mut frame_timer: FrameTimer = FrameTimer::new();
        let mut events_loop = glutin::EventsLoop::new();
        let mut store_frame = None;

        let window = glutin::WindowBuilder::new()
            .with_title("Storm and Swords")
            .with_dimensions(800, 1000);
        let context = glutin::ContextBuilder::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current() }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);


        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut square = SquareRenderer::new();
        let mut circle = CircleRenderer::new();
    //    let mut frame = FrameRenderer::new();

        let mut running = true;
        while running {

            frame_timer.frame_start();

            events_loop.poll_events(|event| {

                if let glutin::Event::WindowEvent { event, .. } = event {
                    match event {
                        glutin::WindowEvent::Closed => running = false,
                        glutin::WindowEvent::KeyboardInput {
                            device_id: _id_of_device,
                            input: input_event,
                        } => {
                            if input_event.virtual_keycode.is_none() == true {
                                return;
                            }

                            if input_event.virtual_keycode.unwrap() == VirtualKeyCode::Escape {
                                running = false;
                            }

                            let _ = self.to_game_thread_with_input.send(
                                input_event,
                            );
                        },
                        glutin::WindowEvent::Resized(_width, _height) => {
                            // TODO
                        },
                        _ => (),
                    }
                }
            });

            
            unsafe {
       //         gl::BindFramebuffer(gl::FRAMEBUFFER, frame.frame_buffer_name);
        //        gl::Viewport(0, 0, 800, 1000);
                
                gl::ClearColor(0.16, 0.5, 0.72, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            };

            let frame_data = self.from_game_thread.try_recv();

            let mut frame_data = match frame_data {
                Ok(data) => Some(data),
                Err(_) => None,
            };

            if frame_data.is_some() {
                store_frame = frame_data.clone();
            } else {
                frame_data = store_frame.clone();
            }

            if frame_data.is_some() {
                let frame_data = frame_data.unwrap();

                if frame_data.sqaures.is_some() {
                    square.render(&frame_data.sqaures.unwrap(), &self);
                }

                if frame_data.circles.is_some() {
                    circle.render(&frame_data.circles.unwrap(), &self);
                }
            }

  //          frame.render();        
            let _ = gl_window.swap_buffers();

            frame_timer.frame_end();
        }


    }
}
