#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

pub mod game;
pub mod graphics;
pub mod math;
pub mod physics;

use graphics::vertex::Vertex;
use game::world::World;
use game::entity::Entity;
use game::sprite_component::SpriteComponent;
use graphics::sprite::Sprite;
use graphics::renderable::Renderable;

fn main() {


    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

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
    /*
    let sprite_component = entity_back.get_component("SpriteComponent".to_string()).unwrap();
    let sprite_component = sprite_component as &Box<SpriteComponent>;
    */

    println!("{:?}", display.get_opengl_version());
    println!("{:?}", display.get_supported_glsl_version());

    loop {

        let mut target = display.draw();
        {
            target.clear_color(0.0, 0.0, 1.0, 1.0);
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