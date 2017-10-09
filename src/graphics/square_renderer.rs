use gl;
use gl::types::*;
use graphics::shaders::Shaders;
use std::str;
use std::mem;
use std::ptr;
use std::ffi::CString;
use graphics::renderer::Renderer;

#[derive(Clone)]
pub struct SquareRenderData {
    pub pos: [GLfloat; 2],
    pub height: GLfloat,
    pub width: GLfloat,
    pub color: [GLfloat; 3],
}

pub struct SquareRenderer {
    shader_program: GLuint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
    color_buffer: GLuint,
}

impl SquareRenderer {
    pub fn new() -> SquareRenderer {

        let frag = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/square_shader.vs"
            ))).unwrap(),
            gl::VERTEX_SHADER,
        );

        let vert = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/square_shader.fs"
            ))).unwrap(),
            gl::FRAGMENT_SHADER,
        );

        let mut vertex_buffer = 0;
        let mut index_buffer = 0;
        let mut color_buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::GenBuffers(1, &mut index_buffer);
            gl::GenBuffers(1, &mut color_buffer);
        }

        SquareRenderer {
            shader_program: Shaders::link_shaders(vert, frag),
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            color_buffer: color_buffer,
        }
    }

    pub fn render(&mut self, sqaures: &Vec<SquareRenderData>, main_renderer: &Renderer) {

        let mut vertex_array: Vec<GLfloat> = vec![];
        let mut index_array: Vec<GLuint> = vec![];
        let mut color_array: Vec<[GLfloat; 3]> = vec![];
        let mut count = 0;

        for sqd in sqaures {
            vertex_array.extend(
                &[
                    (-0.5 * sqd.width) + sqd.pos[0],
                    (-0.5 * sqd.height) + sqd.pos[1],
                    (0.5 * sqd.width) + sqd.pos[0],
                    (-0.5 * sqd.height) + sqd.pos[1],
                    (-0.5 * sqd.width) + sqd.pos[0],
                    (0.5 * sqd.height) + sqd.pos[1],
                    (0.5 * sqd.width) + sqd.pos[0],
                    (0.5 * sqd.height) + sqd.pos[1],
                ],
            );

            color_array.extend(&[sqd.color, sqd.color, sqd.color, sqd.color]);

            index_array.extend(
                &[
                    0 + count,
                    1 + count,
                    2 + count,
                    2 + count,
                    1 + count,
                    3 + count,
                ],
            );
            count += 4;
        }


        //setup shader program
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindFragDataLocation(
                self.shader_program,
                0,
                CString::new("out_color").unwrap().as_ptr(),
            );

            let matrix_id = gl::GetUniformLocation(self.shader_program, CString::new("ortho").unwrap().as_ptr());
            gl::UniformMatrix4fv(
                matrix_id,
                1,
                gl::FALSE as GLboolean,
                &main_renderer.ortho_matrix.x[0] as *const f32,
            );
        }

        //fill buffers
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(vertex_array.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0 as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (color_array.len() * mem::size_of::<GLfloat>() * 3) as GLsizeiptr,
                mem::transmute(color_array.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_array.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                mem::transmute(index_array.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::DrawElements(
                gl::TRIANGLES,
                index_array.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

        }


    }
}
