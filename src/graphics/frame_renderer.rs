use gl;
use gl::types::*;
use graphics::shaders::Shaders;
use std::str;
use std::mem;
use std::ptr;
use std::ffi::CString;
use graphics::renderer::Renderer;


pub struct FrameRenderer {
    pub frame_buffer: GLuint,
    pub render_texture: GLuint,
    pub depth_render_buffer: GLuint,
}

impl FrameRenderer {
    pub fn new() -> FrameRenderer {
        let mut frame_buffer_name = 0;
        gl::GenFramebuffers(1, &mut frame_buffer_name);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer_name);
        
        let mut rendered_texture = 0;
        gl::GenTextures(1, &mut rendered_texture);
        gl::BindTexture(gl::TEXTURE_2D, rendered_texture);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 1024, 768, 0, gl::RGB, gl::UNSIGNED_BYTE, 0 as GLsizeiptr);

        FrameRenderer {
            
        }
    }
}