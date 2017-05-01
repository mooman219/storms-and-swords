#![feature(test)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;
extern crate test;
extern crate threadpool;

pub mod game;
pub mod graphics;
pub mod math;
pub mod physics;
pub mod content;



use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


use content::load_content::{EContentType, EContentRequestType, EContentRequestResult, EContentLoadRequst};
use content::{ContentManifest, LoadContent};
use graphics::RenderThread;
use graphics::render_thread::RenderFrame;
use game::{World, ContentId};

fn main() {

  
    //this is for assets that have been loaded by their threads
    //and then for the content manifest to keep track of them
    let (load_subthread_sender, content_manifest_asset_receiver): (Sender<EContentType>,
                                                                   Receiver<EContentType>) =
        mpsc::channel();

    //this is for the game thread to ask for an asset to be loaded
    //and for the load thread to kick off the loading process
    let (game_thread_request, loading_thread_fulfillment): (Sender<EContentRequestType>,
                                                            Receiver<EContentRequestType>) =
        mpsc::channel();

    //this is for the render thread to ask the content manifest for an asset
    //and for the content manifest to handle that request
    let (render_thread_asset_request, content_manifest_request_fulfillment)
            : (Sender<EContentLoadRequst>, Receiver<EContentLoadRequst>)
            = mpsc::channel();

    //this is for the content manifest to send assets that the loading thread has asked for
    //and for the render thread to start using them
    let (content_manifest_fulfillment, render_thread_asset_reciver): (Sender<EContentType>,
                                                                      Receiver<EContentType>) =
        mpsc::channel();

    //this is for the loading thread to send back the content id associated with the asset that the
    //game just asked for
    let (loading_thread_content_id, game_thread_content_id): (Sender<EContentRequestResult>,
                                                              Receiver<EContentRequestResult>) =
        mpsc::channel();

    //this is for the game thread to use to send over frames it wants rendered
    let (game_thread_render_frame, render_thread_render_frame): (Sender<RenderFrame>,
                                                                 Receiver<RenderFrame>) =
        mpsc::channel();

    let _ = thread::spawn(move || {
        ContentManifest::thread_loop(content_manifest_asset_receiver,
                                     content_manifest_request_fulfillment,
                                     content_manifest_fulfillment.clone())
    });

    
    //create a content loader
    let load_content = LoadContent::new(loading_thread_fulfillment,
                                            loading_thread_content_id.clone(),
                                            load_subthread_sender.clone());

    let _ = thread::spawn(move || {
        LoadContent::thread_loop(load_content);
    });
    
    //create a render loop
    let _ = thread::spawn(move || {
        RenderThread::thread_loop(render_thread_render_frame,
                                  render_thread_asset_request.clone(),
                                  render_thread_asset_reciver);
    });

    //create a game thread
    let world = World::new(game_thread_request, 
                               game_thread_content_id,
                               game_thread_render_frame.clone());

    let _ = thread::spawn(move || {
        World::update(world);
    });

    

    /*
    let mut world = World::new();
    let entity_uid = world.get_uid();
    let entity = Entity::new("Test entity".to_string(), entity_uid.clone());
    let sprite = Sprite::new("image.png".to_string(), &display);
    let sprite_component = SpriteComponent::new(sprite,
                                                    Vertex{position: [-0.5,  0.5], tex_coords: [1.0, 0.0]},
                                                    Vertex{position: [ 0.5,  0.5], tex_coords: [1.0, 1.0]},
                                                    Vertex{position: [-0.5, -0.5], tex_coords: [0.0, 0.0]},
                                                    Vertex{position: [ 0.5, -0.5], tex_coords: [0.0, 1.0]},
                                                    &display);
                                                
    world.add_entity(entity);
    let entity_back = world.get_entity(entity_uid).unwrap();
    let sprite_component = entity_back.get_component("SpriteComponent".to_string()).unwrap();
    let sprite_component = sprite_component as &Box<SpriteComponent>;
    */

    //  println!("{:?}", display.get_opengl_version());
    //println!("{:?}", display.get_supported_glsl_version());

    /*
    loop {

        let mut target = display.draw();
        {
            target.clear_color(1.0, 0.5, 0.1, 1.0);
        }

        sprite_component.render(&entity_back, display.draw());

        
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }

    }
    */
    /*
    let image = image::load(Cursor::new(&include_bytes!("../opengl.png")[..])
                                        ,image::PNG).unwrap().to_rgb();

    let image_dimesions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgb_reversed(image.into_raw(), image_dimesions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let vertex1 = Vertex{position: [-0.5, -0.5], tex_coords: [0.0, 0.0]};
    let vertex2 = Vertex{position: [ 0.0,  0.5], tex_coords: [0.0, 1.0]};
    let vertex3 = Vertex{position: [ 0.5, -0.5], tex_coords: [1.0, 0.0]};
    
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display,&shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix *  vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    loop {
        t += 0.002;
        if t > 0.5{
            t = -0.5;
        }

        let uniform = uniform!{matrix:[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [t, 0.0, 0.0, 1.0f32],
            
        ],
            tex: &texture,
        };




        
    }
    */
}
