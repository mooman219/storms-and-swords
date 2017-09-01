
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{SystemTime, Duration};
use std::thread::sleep;

use std::ops::Mul;

/*
type Matrix = [[f32; 4]; 4];

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {

        for x in 0..4 {
            for y in 0..4 {

            }
        }

    }
}
*/

/*
use std::time::Duration;

trait AsMillis {
    fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
    fn as_millis(&self) -> u64 {
        self.as_secs() * 1000 + (self.subsec_nanos() / 1_000_000u32) as u64
    }
}

fn main() {
    let duration = Duration::from_millis(5010);
    assert_eq!(duration.as_millis(), 5010);
}
*/

pub use gfx_app::{ColorFormat, DepthFormat};
use cgmath::{Deg, Matrix4, Point3, Vector3};
use gfx::{Bundle, texture};
use gfx;
use glutin;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use graphics::sprite_renderer::{SpriteRenderer, SpriteRenderData};
use glium::{self, DisplayBuild};
use glium::backend::glutin_backend::GlutinFacade;
use game::ContentId;
use content::load_content::{EContentType, EContentLoadRequst};
use glium::texture::Texture2d;
use graphics::sprite::Sprite;
use graphics::vertex::{pipe, Vertex};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const SHOW_BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const ONE_FRAME_IN_MILLIE: u64 = 1000 / 60;

gfx_defines! {
    vertex VertexColor {
        pos: [f32;2] = "a_Pos",
        color: [f32;3] = "a_Color",
    }

    pipeline pipe_color {
        vbuf: gfx::VertexBuffer<VertexColor> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }

    constant Transform {
        transform:  [[f32;4]; 4] = "u_Transform",
        scale:      [[f32;4]; 4] = "u_Scale",
        rotation_z: [[f32;4]; 4] = "u_Rotation_z",
    }

    pipeline pipe_sin {
        vbuf: gfx::VertexBuffer<VertexColor> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const SQAURE: [VertexColor; 3] = [
    VertexColor {
        pos: [0.5, -0.5],
        color: WHITE,
    },
    VertexColor {
        pos: [-0.5, -0.5],
        color: SHOW_BLACK,
    },
    VertexColor {
        pos: [-0.5, 0.5],
        color: WHITE,
    },
];

const OTHER_SQAURE: [VertexColor; 3] = [
    VertexColor {
        pos: [0.0, -0.5],
        color: RED,
    },
    VertexColor {
        pos: [-1.0, -0.5],
        color: GREEN,
    },
    VertexColor {
        pos: [-1.0, 0.5],
        color: BLUE,
    },
];

const BOX: [VertexColor; 4] = [
    VertexColor {
        pos: [-0.5, -0.5],
        color: GREEN,
    },
    VertexColor {
        pos: [0.5, -0.5],
        color: BLUE,
    },
    VertexColor { 
        pos: [-0.5, 0.5],
        color: SHOW_BLACK,
    },
    VertexColor {
        pos: [0.5, 0.5],
        color: RED,
    }
];


#[derive(Clone)]
pub struct RenderFrame {
    pub sprite_renderers: Vec<SpriteRenderData>,
    pub frame_index: u64, //we keep track so we know in what relation we are to the main game loop
}

impl RenderFrame {
    pub fn new(frame_index: u64) -> RenderFrame {
        RenderFrame {
            sprite_renderers: vec![],
            frame_index: frame_index,
        }
    }
}

pub struct RenderThread {
    from_game_thread: Receiver<RenderFrame>,
    to_content_manifest: Sender<EContentLoadRequst>,
    from_content_manifest: Receiver<EContentType>,
    _current_frame_index: u64,
    //   sprite_renderer: SpriteRenderer,
    sprites: HashMap<ContentId, Sprite>,
}

impl RenderThread {
    pub fn new(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
    ) -> RenderThread {

        //  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        //let sprite_renderer = SpriteRenderer::new(&display);

        RenderThread {
            //      display: display,
            _current_frame_index: 0,
            //    sprite_renderer: sprite_renderer,
            from_game_thread: from_game_thread,
            to_content_manifest: to_content_manifest,
            from_content_manifest: from_content_manifest,
            sprites: HashMap::new(),
        }
    }

    pub fn query_content_manifest_for_sprite(&mut self, content_id: ContentId) -> bool {
        if self.sprites.contains_key(&content_id) {
            true
        } else {
            let _ = self.to_content_manifest.send(EContentLoadRequst::Image(
                content_id,
            ));
            let value = self.from_content_manifest.recv().unwrap();
            match value {
                EContentType::Image(id, dy_image) => {
                    /*
                    let image_dimensions = dy_image.to_rgba().dimensions();
                    let loaded_image = glium::texture::RawImage2d::from_raw_rgba_reversed(dy_image.to_rgba().into_raw(), image_dimensions);
                    let tex = Texture2d::new(&self.display, loaded_image).unwrap();
                    let spr = Sprite::new("Sprite".to_string(), tex, &self.display);
                    self.sprites.insert(id, spr);
                    */

                    true
                }
                EContentType::NotLoaded => false,
            }
        }

    }

    pub fn get_sprite(&self, content_id: ContentId) -> Option<&Sprite> {
        if self.sprites.contains_key(&content_id) {
            return self.sprites.get(&content_id);
        }
        None
    }

    pub fn thread_loop(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
    ) {


        let mut rend =
            RenderThread::new(from_game_thread, to_content_manifest, from_content_manifest);

        rend.render();
    }


    pub fn render(&mut self) {
        let TOTAL_FRAME_DURATION = Duration::from_millis(8);
        let events_loop = glutin::EventsLoop::new();

        let builder = glutin::WindowBuilder::new()
            .with_title("Square Toy".to_string())
            .with_dimensions(800, 800)
            .with_vsync();

        let (window, mut device, mut factory, mut main_color, mut main_depth) =
            gfx_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

      //  let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        let mut encoder_for_sin: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = factory
            .create_pipeline_simple(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_150.glslv"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_150.glslf"
                )),
                pipe_color::new(),
            )
            .unwrap();

        let pso_inverse = factory
            .create_pipeline_simple(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_inverse_150.glslv"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/rect_inverse_150.glslf"
                )),
                pipe_sin::new(),
            )
            .unwrap();

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQAURE, ());
        let (vertex_buffer_other, slice_other) =
            factory.create_vertex_buffer_with_slice(&OTHER_SQAURE, ());


        let BOX_INDEX: Vec<u16> = vec![0u16, 1, 2, 2, 1, 3];

        let (box_vertex_buffer, box_index_buffer) = factory.create_vertex_buffer_with_slice(&BOX, &*BOX_INDEX);

        /*
        let x_rot_matrix : Matrix4<f32> = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0,  rotation.x.cos(), -rotation.x.sin(), 0.0,
            0.0,  rotation.x.sin(),  rotation.x.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        let y_rot_matrix : Matrix4<f32> = Matrix4::new(
            rotation.y.cos(), -rotation.y.sin(), 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 
            -rotation.y.sin(), 0.0, rotation.x.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        
        let z_rot_matrix : Matrix4<f32> = Matrix4::new(
            rotation.z.cos(), -rotation.z.sin(), 0.0, 0.0,
            rotation.z.sin(), rotation.z.cos(),  0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        */


        let mut TRANSFORM: Transform = Transform {
            transform: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.3, 0.2, 0.0, 1.0]],

            scale:     [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],

            rotation_z:[[f32::cos(45.0), f32::sin(45.0), 0.0, 0.0],
                        [-f32::sin(45.0), f32::cos(45.0), 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],            
            };


        let mut box_data = pipe_color::Data {
            vbuf: vertex_buffer.clone(),
            out: main_color.clone(),
        };

        let mut other_data = pipe_color::Data {
            vbuf: vertex_buffer_other.clone(),
            out: main_color.clone(),
        };

    /*
        let mut sin_value = 0;
        let mut sin_data = pipe_sin::Data {
            vbuf: vertex_buffer_other.clone(),
            transform: transform_buffer,
            sin_num: sin_value.clone(),
            out: main_color.clone(),
        };
*/
        
        let transform_buffer = factory.create_constant_buffer(1);
        let mut data = pipe_sin::Data {
            vbuf: box_vertex_buffer.clone(),
            transform: transform_buffer,
            out: main_color.clone()
        };

        encoder_for_sin.update_buffer(&data.transform, &[TRANSFORM], 0);

        let mut running = true;
        let mut once = false;
        let mut angle: f32 = 0f32;
        let mut frame_start;


        while running {
            
            frame_start = SystemTime::now();
            if once {
                continue;
            }
            //the first thing we do is grab the current frame
            let frame_data = self.from_game_thread.try_recv();
            events_loop.poll_events(|glutin::Event::WindowEvent {
                 window_id: _,
                 event,
             }| {
                use glutin::WindowEvent::*;
                use glutin::{MouseButton, ElementState, VirtualKeyCode};
                match event {
                    KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                    Closed => running = false,
                    Resized(w, h) => {
                        gfx_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    }
                    _ => (),
                }
            });

       //     encoder.clear(&data.out, BLACK);
         //  encoder.draw(&slice, &pso, &data);
         //   encoder.flush(&mut device);

             encoder_for_sin.clear(&data.out, BLACK);
             encoder_for_sin.draw(&box_index_buffer, &pso_inverse, &data);
       
     //       encoder_for_sin.draw(&slice_other, &pso_inverse, &sin_data);
     
     //       sin_data.sin_num = sin_data.sin_num + 1;
            TRANSFORM.rotation_z[0][0] = f32::cos(angle);
            TRANSFORM.rotation_z[0][1] = f32::sin(angle);
            TRANSFORM.rotation_z[1][0] = -f32::sin(angle);
            TRANSFORM.rotation_z[1][1] = f32::cos(angle);
            angle = angle + 0.1f32;
            
            
            encoder_for_sin.update_buffer(&data.transform, &[TRANSFORM], 0);

            encoder_for_sin.flush(&mut device);

            window.swap_buffers().unwrap();
            device.cleanup();

            let frame_duration = frame_start.duration_since(frame_start).unwrap();
            //this is gotten from the rust example page about the Duration struct
            if frame_duration.as_secs() > 0 {
                continue;
            }
            //each frame of rendering must take atleast 16ms to finish this ensure that it does, insuring a steady framerate
            if frame_duration.subsec_nanos() < TOTAL_FRAME_DURATION.subsec_nanos() {
                let sleep_duration = TOTAL_FRAME_DURATION - frame_duration;
                sleep(sleep_duration);
            }
        }
    }
}
