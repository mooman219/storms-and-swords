use graphics::sprite::Sprite;
use graphics::renderable::Renderable;
use graphics::vertex::Vertex;
use glium::{self, Frame, VertexBuffer, IndexBuffer};
use glium::Display;
use game::entity::Entity;

pub struct SpriteComponent {
    sprite: Sprite,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
//    entity: Option<Box<Entity>>
}

impl SpriteComponent {
    pub fn new(sprite: Sprite, top_left: Vertex, top_right: Vertex, bottom_left: Vertex, bottom_right: Vertex ,display: &Display)  -> SpriteComponent {
        
        let shape = [top_left, top_right, bottom_left, bottom_right];
        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        
        let indices = [0, 1, 2, 2, 1, 3];
        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                            &indices).unwrap();
        
        SpriteComponent{sprite: sprite, vertex_buffer: vertex_buffer, index_buffer: index_buffer}//, entity: None}
    }
    /*
    pub fn set_entity(&mut self, entity: &Entity) {
        self.entity = Some(Box::new(entity));
    }
    */
}

impl Renderable {
    pub fn render (&self, frame: &mut Frame) {
        
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

