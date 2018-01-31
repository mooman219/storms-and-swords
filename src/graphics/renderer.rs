use std::collections::HashMap;
use std::fs::{self};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

use cgmath::{Matrix4, ortho};

pub const SCREEN_SCALE : f32 = 3.5f32;
pub const BASE_SCREEN_WIDTH : f32 = 1280.0f32;
pub const BASE_SCREEN_HEIGHT : f32 = 720.0f32;

use gl;
use gl::types::*;
use glutin;
use glutin::{GlContext, VirtualKeyCode};
use content::load_content::{EContentType, EContentLoadRequst};
use game::input::*;

use frame_timer::FrameTimer;
use graphics::square_renderer::{SquareRenderData, SquareRenderer};
use graphics::circle_renderer::{CircleRenderData, CircleRenderer};
use graphics::sprite_renderer::{SpriteRenderData, SpriteRenderer, SpriteRecordData};
use serde_json;

#[derive(Clone)]
pub struct RenderFrame {
    pub frame_index: u64,
    pub sqaures: Option<Vec<SquareRenderData>>,
    pub circles: Option<Vec<CircleRenderData>>,
    pub static_sprites: Option<Vec<SpriteRenderData>>,
}

impl RenderFrame {
    pub fn new(frame_index: u64, sqaures: Option<Vec<SquareRenderData>>, 
                                 circles: Option<Vec<CircleRenderData>>,
                                 sprites: Option<Vec<SpriteRenderData>>
                                 ) -> RenderFrame {
        RenderFrame {
            frame_index: frame_index,
            sqaures: sqaures,
            circles: circles,
            static_sprites: sprites
        }
    }
}

pub struct Renderer {
    pub ortho_matrix: Matrix4<GLfloat>,
    from_game_thread: Receiver<RenderFrame>,
    _to_content_manifest: Sender<EContentLoadRequst>,
    _from_content_manifest: Receiver<EContentType>,
    to_game_thread_with_input: Sender<InputMessage>,
    sprite_name_to_texture_id: HashMap<String, String>,
}

impl Renderer {
    pub fn new(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<InputMessage>,
    ) -> Renderer {
        Renderer {
            ortho_matrix: ortho(0.0f32, BASE_SCREEN_WIDTH * SCREEN_SCALE, 0.0f32, BASE_SCREEN_HEIGHT * SCREEN_SCALE, 0.0, 10.0),
            from_game_thread: from_game_thread,
            _to_content_manifest: to_content_manifest,
            _from_content_manifest: from_content_manifest,
            to_game_thread_with_input: to_game_thread_with_input,
            sprite_name_to_texture_id: HashMap::new(),
        }
    }

    pub fn render_thread(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<InputMessage>,
    ) {


        let render_thread = Renderer::new(
            from_game_thread,
            to_content_manifest,
            from_content_manifest,
            to_game_thread_with_input,
        );
        render_thread.render();
    }
    //as this function does a lot, it felt best to have it be its own function
    pub fn load_textures_create_sprite_renderer_and_load_sprite_data(&mut self, static_sprite_renderers: &mut HashMap<String, SpriteRenderer>) {

        use std::fs::File;
        use std::io::prelude::*;

        let sprite_data_dir_path = "./content/sprite_data";

        let p = Path::new("./content/textures");
        if p.is_dir() {
            for entry in fs::read_dir(p).unwrap() {
                
                //making my life easier
                let entry = entry.unwrap();
                let path = entry.path();

                      //we now need to look up the sprite data, should be under the same name as the texture, just with a different
                //file extension .json
                let file_name = entry.file_name().into_string().unwrap();
                let spl : Vec<&str> = file_name.split(".").collect();
                let possible_sprite_data_path = String::from(sprite_data_dir_path) + "/" + &spl[0] + ".json";
                let sprite_data_file = File::open(possible_sprite_data_path.clone());

                match sprite_data_file {
                    Ok(mut file) => {
                        //we have a json sprite data file, consume it, using it to set up the last connection between an enity and the sprite
                        //that they want rendered
                        let mut s = String::new();
                        let _ = file.read_to_string(&mut s);
                        let new_sprite_data_record: SpriteRecordData = serde_json::from_str(&s).unwrap();

                        //create the new renderer, this will load the texture at path
                        let file_name = entry.file_name().into_string().unwrap();

                        for (k, _v) in &new_sprite_data_record.data {
                            self.sprite_name_to_texture_id.insert(k.clone(), file_name.clone());
                        }
                        
                        let new_sprite_renderer = SpriteRenderer::new(path.into_os_string().into_string().unwrap(), new_sprite_data_record);
                        
                        //add it to the renderers
                        static_sprite_renderers.insert(file_name, new_sprite_renderer);
                    },
                    Err(e) => {
                        println!(" error reading {:?} ERROR: {}", possible_sprite_data_path, e);
                    }
                }

                

          

            }
        }
    }

    pub fn render(mut self) {

        let mut frame_timer: FrameTimer = FrameTimer::new();
        let mut events_loop = glutin::EventsLoop::new();
        let mut store_frame = None;

        let window = glutin::WindowBuilder::new()
            .with_title("Storm and Swords")
            .with_dimensions(BASE_SCREEN_WIDTH as u32, BASE_SCREEN_HEIGHT as u32);
        let context = glutin::ContextBuilder::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
        let hidpi_scale_factor =  gl_window.hidpi_factor();

        unsafe { gl_window.make_current() }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);


        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);  
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut square = SquareRenderer::new();
        let mut circle = CircleRenderer::new();
        let mut static_sprite_renderers = HashMap::new();

        
        //go through each texture in the texture folder, creating a SpriteRenderer for it
        self.load_textures_create_sprite_renderer_and_load_sprite_data(&mut static_sprite_renderers);
        //each SpriteRenderer handles loading the texture into memory
        //this way they have complete owernship of the sheet that they are tasked with rendereing
        //then look at each of file names and look for corresponding files in the sprite_data folder
        //build the sprite data map with this, allowing 
        //key == sprite_name, value = Tuple(SpriteSheetName, SpriteSheetBoxData)
        //then when a sprite wants to get renderered it 

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
                                InputMessage::KeyboardEvent(input_event),
                            );
                        },
                        glutin::WindowEvent::CursorMoved{ 
                            position,
                            ..
                        } => {
                            //hidpi will create problems with mac monitors thinking that they are twice the size that they think they are
                            //we divide the sampled mouse positions by it, which will leave it alone (mouse_pos/1) for regular monitors
                            //cut it in have for monitors that have this problem 
                            let adjust_for_scale_factor = (position.0 / hidpi_scale_factor as f64, position.1 / hidpi_scale_factor as f64);
                            let _ = self.to_game_thread_with_input.send(InputMessage::CursorEvent(adjust_for_scale_factor));
                        },
                        glutin::WindowEvent::Resized(_width, _height) => {
                            // TODO
                        },
                        _ => (),
                    }
                }
            });



            
            unsafe {
                gl::ClearColor(0.16, 0.5, 0.72, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
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
                if frame_data.static_sprites.is_some() {
                    let static_sprites = frame_data.static_sprites.unwrap();
                    for sprite in static_sprites {
                        //this is a many to one, to one relationship of many sprites, to one texture, to one sprite renderer
                        //I Would enjoy being able to cut out the middle step there
                        //TODO: simplify thisc
                        let sprite_sheet_name = self.sprite_name_to_texture_id.get(&sprite.sprite_name).unwrap();
                        let sprite_sheet_renderer = static_sprite_renderers.get_mut(sprite_sheet_name).unwrap();
                        sprite_sheet_renderer.current_sprite_datas.push(sprite);
                    }

                    for (_k, v) in &mut static_sprite_renderers {
                        v.render(&self);
                    }

                }
            }

            let _ = gl_window.swap_buffers();

            frame_timer.frame_end();
        }


    }
}
