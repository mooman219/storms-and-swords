use glium::texture::Texture2d;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{VertexBuffer, IndexBuffer};
use graphics::vertex::Vertex;


pub struct Sprite {
    texture: Texture2d,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}

impl Sprite {
    pub fn new(_image_name: String, texture: Texture2d, display: &GlutinFacade) -> Sprite {

        let bl = Vertex {
            position: [-0.5, 0.5],
            tex_coords: [1.0, 0.0],
        };
        let br = Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        };
        let tl = Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        };
        let tr = Vertex {
            position: [0.5, -0.5],
            tex_coords: [0.0, 1.0],
        };

        let shape = [tl, tr, bl, br];
        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();

        let indices = [0, 1, 2, 2, 1, 3];
        let index_buffer = IndexBuffer::new(display,
                                            glium::index::PrimitiveType::TrianglesList,
                                            &indices)
            .unwrap();

        Sprite {
            texture: texture,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }

    pub fn get_texture(&self) -> &Texture2d {
        &self.texture
    }

    pub fn get_vertex_buffer(&self) -> &VertexBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &IndexBuffer<u16> {
        &self.index_buffer
    }
}
