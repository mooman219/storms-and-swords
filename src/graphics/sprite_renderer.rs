use cgmath::Vector3;
use game::ContentId;
use glium::{self, Frame};
use glium::{Display, Surface};
use cgmath::Matrix4;
use graphics::sprite::Sprite;


/*
*    Each character in game that wants to be rendered must present one of these perframe that it wants to be rendered
*/
#[derive(Clone)]
pub struct SpriteRenderData {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub sprite: ContentId,
}

pub struct SpriteRenderer {
    sprite_shader: glium::Program,
}

impl SpriteRenderer {
    pub fn new(display: &Display) -> SpriteRenderer {
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

        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        SpriteRenderer { sprite_shader: program }
    }

    pub fn render(&self, spr: &SpriteRenderData, sprite: &Sprite, mut frame: &mut Frame) {
        /*
        let translation_matrix: Matrix4<f32> = Matrix4::new(1.0,
                                                            0.0,
                                                            0.0,
                                                            0.0,
                                                            0.0,
                                                            1.0,
                                                            0.0,
                                                            0.0,
                                                            0.0,
                                                            0.0,
                                                            1.0,
                                                            0.0,
                                                            spr.pos.x,
                                                            spr.pos.y,
                                                            spr.pos.z,
                                                            1.0f32);

        let scale_matrix: Matrix4<f32> = Matrix4::new(spr.scale.x,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      spr.scale.y,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      spr.scale.z,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0f32);

        let x_rot_matrix: Matrix4<f32> = Matrix4::new(1.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      spr.rotation.x.cos(),
                                                      -spr.rotation.x.sin(),
                                                      0.0,
                                                      0.0,
                                                      spr.rotation.x.sin(),
                                                      spr.rotation.x.cos(),
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0);

        let y_rot_matrix: Matrix4<f32> = Matrix4::new(spr.rotation.y.cos(),
                                                      -spr.rotation.y.sin(),
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0,
                                                      0.0,
                                                      0.0,
                                                      -spr.rotation.y.sin(),
                                                      0.0,
                                                      spr.rotation.x.cos(),
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0);

        let z_rot_matrix: Matrix4<f32> = Matrix4::new(spr.rotation.z.cos(),
                                                      -spr.rotation.z.sin(),
                                                      0.0,
                                                      0.0,
                                                      spr.rotation.z.sin(),
                                                      spr.rotation.z.cos(),
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      0.0,
                                                      1.0);

        let mut result_matrix = translation_matrix * scale_matrix;
        let rotation_mul = x_rot_matrix * y_rot_matrix * z_rot_matrix;
        result_matrix = result_matrix * rotation_mul;

        let uni = uniform!{
            
            matrix:[[result_matrix.x.x, result_matrix.x.y, result_matrix.x.z, result_matrix.x.w],
                    [result_matrix.y.x, result_matrix.y.y, result_matrix.y.z, result_matrix.y.w],
                    [result_matrix.z.x, result_matrix.z.y, result_matrix.z.z, result_matrix.z.w],
                    [result_matrix.w.x, result_matrix.w.y, result_matrix.w.z, result_matrix.w.w]
                    ],
            tex: sprite.get_texture()
        };
        let params =
            glium::DrawParameters { blend: glium::Blend::alpha_blending(), ..Default::default() };

        frame.draw(sprite.get_vertex_buffer(),
                  sprite.get_index_buffer(),
                  &self.sprite_shader,
                  &uni,
                  &params)
            .unwrap();
                */
    }
    
}

/*use graphics::sprite::Sprite;
use graphics::renderable::Renderable;
use game::entity::Entity;
use game::component::Component;

pub struct SpriteComponent {
    sprite: Sprite,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    
}

impl SpriteComponent {
    pub fn new(sprite: Sprite, top_left: Vertex, top_right: Vertex, bottom_left: Vertex, bottom_right: Vertex ,display: &Display)  -> SpriteComponent {
        
        let shape = [top_left, top_right, bottom_left, bottom_right];
        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        
        let indices = [0, 1, 2, 2, 1, 3];
        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                            &indices).unwrap();

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

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        
        SpriteComponent{sprite: sprite,
                        vertex_buffer: vertex_buffer,
                        index_buffer: index_buffer,
                        sprite_shader: program}
    }
}

impl Component for SpriteComponent {

    fn get_name(&self) -> String {
        return "SpriteComponent".to_string();
    }

    fn to_box(self) -> Box<Component> {
        Box::new(self)
    }
    
}

impl Renderable for SpriteComponent {
    fn render (&self, entity: &Entity, mut frame: Frame) {
        
        let pos = entity.get_position();

        let translation_matrix =  Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            pos.x, pos.y, pos.z, 1.0f32
        );

        let scale = entity.get_scale();
        let scale_matrix: Matrix4<f32> = Matrix4::new(
            scale.x, 0.0, 0.0, 0.0,
            0.0, scale.y, 0.0, 0.0,
            0.0, 0.0, scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0f32
        );

        let rotation = entity.get_rotation();
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
        
        let mut result_matrix = translation_matrix * scale_matrix;
        let rotation_mul = x_rot_matrix * y_rot_matrix * z_rot_matrix;
        result_matrix = result_matrix * rotation_mul;
        
        let uni = uniform!{
            
            matrix:[[result_matrix.x.x, result_matrix.x.y, result_matrix.x.z, result_matrix.x.w],
                    [result_matrix.y.x, result_matrix.y.y, result_matrix.y.z, result_matrix.y.w],
                    [result_matrix.z.x, result_matrix.z.y, result_matrix.z.z, result_matrix.z.w],
                    [result_matrix.w.x, result_matrix.w.y, result_matrix.w.z, result_matrix.w.w]
                    ],
            tex: self.sprite.get_texture()
        };
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        frame.draw(&self.vertex_buffer, &self.index_buffer, &self.sprite_shader, &uni, &params).unwrap();

        frame.finish().unwrap();
       // frame.finish().unwrap();
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

*/
