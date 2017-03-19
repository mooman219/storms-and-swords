use graphics::sprite::Sprite;
use graphics::renderable::Renderable;
use graphics::vertex::Vertex;
use glium::{self, Frame, VertexBuffer, IndexBuffer};
use glium::Display;
use game::entity::Entity;
use cgmath::{Vector3, Matrix4};

pub struct SpriteComponent {
    sprite: Sprite,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    entity: Option<Box<Entity>>
}

impl SpriteComponent {
    pub fn new(sprite: Sprite, top_left: Vertex, top_right: Vertex, bottom_left: Vertex, bottom_right: Vertex ,display: &Display)  -> SpriteComponent {
        
        let shape = [top_left, top_right, bottom_left, bottom_right];
        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        
        let indices = [0, 1, 2, 2, 1, 3];
        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                            &indices).unwrap();
        
        SpriteComponent{sprite: sprite, vertex_buffer: vertex_buffer, index_buffer: index_buffer, entity: None}
    }
    
    /*
    pub fn set_entity(&mut self, entity: &Entity) {
        self.entity = Some(Box::new(entity));
    }
    */

}

impl Renderable for SpriteComponent {
    fn render (&self, frame: &mut Frame) {
        
        match self.entity {
            Some(ref entity) => {
                let pos = entity.get_position();

                let translation_matrix: Matrix4<f32> = Matrix4::new(
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    pos.x, pos.y, pos.z, 1.0f32
                );

                let scale = entity.get_scale();
                let scale_matrix: Matrix4<f23> = Matrix4::new(
                    scale.x, 0.0, 0.0, 0.0,
                    0.0, scale.y, 0.0, 0.0,
                    0.0, 0.0, scale.z, 0,0,
                    0.0, 0.0, 0.0, 1.0f32
                );

                let rotation = entity.get_rotation();
                let x_rot_matrix = Matrix4<f32> = Matrix4::new(
                    1.0, 0.0, 0,0, 0.0,
                    0.0,  rotation.x.cos(), -rotation.x.sin(), 0.0,
                    0.0,  rotation.x.sin(),  rotation.x.cos(), 0.0,
                    0.0, 0.0, 0.0, 1.0
                );

                let y_rot_matrix = Matrix4<f32> = Matrix4::new(
                    rotation.y.cos(), -rotation.y.sin(), 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0, 
                    -rotation.y.sin(), 0.0, rotation.x.cos(), 0.0,
                    0.0, 0.0, 0.0, 1.0
                ):

                let z_rot_matrix = Matrix4<f32> = Matrix4::new() {
                    rotation.z.cos(), -rotation.z.sin(), 0.0, 0.0,
                    rotation.z.sin(), rotation.z.cos(),  0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                };

                let mut result_matrix = translation_matrix * scale;

            },
            None => {
                println!("Cannot render");
            }
        }
        //program -> this I can almost do at compile time
        /*
        let translation = 
        let uniform = uniform!{

            //matrix
            /*
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [t, 0.0, 0.0, 1.0f32],
            */
            //tex


        };
        */
        //uniform that I do need to self
        
        //


        //frame.draw()
    }
}

