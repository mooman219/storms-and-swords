use gl;
use gl::types::*;
use graphics::shaders::Shaders;
use std::str;
use std::mem;
use std::ptr;
use std::ffi::CString;
///use graphics::renderer::Renderer;


pub struct FrameRenderer {
    pub frame_buffer_name: GLuint,
    pub rendered_texture: GLuint,
    pub depth_render_buffer: GLuint,
    pub shader_program: GLuint,
    pub texture_shader_id: GLint,
    pub quad_vertex_buffer: GLuint,
}


const QUAD_DATA: [f32; 18] = [
            -1.0f32, -1.0f32, 0.0f32,
             1.0f32, -1.0f32, 0.0f32,
            -1.0f32,  1.0f32, 0.0f32,
            -1.0f32,  1.0f32, 0.0f32,
             1.0f32, -1.0f32, 0.0f32,
             1.0f32,  1.0f32, 0.0f32 
];

impl FrameRenderer {
    pub fn new() -> FrameRenderer {

        let frag = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/full_screen_shader.vs"
            ))).unwrap(),
            gl::VERTEX_SHADER,
        );

        let vert = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/full_screen_shader.fs"
            ))).unwrap(),
            gl::FRAGMENT_SHADER,
        );


        let mut frame_buffer_name = 0;
        let mut rendered_texture = 0;
        let mut depth_render_buffer = 0;
        let mut quad_vertex_buffer = 0;
        
        unsafe {
            gl::GenFramebuffers(1, &mut frame_buffer_name);
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer_name);
            
            gl::GenTextures(1, &mut rendered_texture);
            gl::BindTexture(gl::TEXTURE_2D, rendered_texture);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 800, 1000, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            
            gl::GenRenderbuffers(1, &mut depth_render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_render_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT , 800, 1000);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_render_buffer);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, rendered_texture, 0);
        
            gl::GenBuffers(1, &mut quad_vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (QUAD_DATA.len() * mem::size_of::<GLfloat>() * 3) as GLsizeiptr,
                mem::transmute(QUAD_DATA.as_ptr()),
                gl::STATIC_DRAW,
            );

            let draw_buffers : [GLenum; 1] = [gl::COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, draw_buffers.as_ptr());
        }
       
        let shader_program = Shaders::link_shaders(vert, frag);
       
        let texture_shader_id;
       
        unsafe {
            texture_shader_id = gl::GetUniformLocation(shader_program, CString::new("renderedTexture").unwrap().as_ptr()); 
        }
       
        FrameRenderer {
            frame_buffer_name,
            rendered_texture,
            depth_render_buffer,
            shader_program,
            texture_shader_id,
            quad_vertex_buffer
        }
    }


    pub fn render(&mut self)  {
        
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, 800, 1000);
            
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(self.shader_program);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.rendered_texture);
            gl::Uniform1i(self.texture_shader_id, 0);


            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vertex_buffer);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                0, 
                ptr::null()
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}