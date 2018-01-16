use gl;
use gl::types::*;
use graphics::shaders::Shaders;
use std::str;
use std::mem;
use std::ptr;
use std::ffi::CString;
use graphics::renderer::Renderer;
use std;
use std::collections::HashMap;
use image;
use image::*;
use image::{GenericImage, DynamicImage};
//use serde_derive;

#[derive(Clone)]
pub struct SpriteRenderData {
    pub pos: [GLfloat; 2],
    pub sprite_name: String,
    pub height: GLfloat,//most times height, and width are likly to be 1, unless you wish to scale the sprite
    pub width: GLfloat,
    pub depth: f32,//this will be negative, more negative the more thnigs will be drawn ontop of it
    pub reverse_x: bool
}

/*
  A Sprite Sheet has many sprites on it
  A Sprite Sheet has a Sprite Sheet Describtor File, which is a json file
  That has a list of sprites on the sprite sheet, and they are
  in the format 
    [
        {
            "name":"foo",
            "x":0,
            "y":0,
            "height":100,
            "weight":100
        },
    ]
    Each entity that wants to use a sprite to be rendered uses the name
    That the sprite is under in the Sprite Sheet Descibtor File
    This means that each sprites needs a unique name, and that
    this must MUST be machine inforced, so a sprite tool
    needs to be written that makes sure that each sprite name is unique
    or that sprite names gain the name of the the sprite sheet they are 
    apart of, the second approch seems to be the better one
*/


//this is not what a game object talks about
//this is for the renderering system, which maintains a hashmap between sprite names, and these boxes
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteRenderBoxData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}


//A SpriteRecordData contains a list of SpriteRenderBoxData
//Which a SpriteRenderData will look up later with its sprite_name field
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteRecordData {
    pub data : HashMap<String, SpriteRenderBoxData>,
}

pub struct SpriteRenderer {
    pub current_sprite_datas: Vec<SpriteRenderData>,
    sprites_on_texture: SpriteRecordData,
    shader_program: GLuint,
    texture_index: GLuint,
    sampler_uniform_location: GLint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
    uv_buffer: GLuint,
    image: DynamicImage,
}


/*
sprite_shaders work a little different
while most renderes work with the idea that all objects of that renderers type need only one to draw all of them
sprites are based on sprite sheets, and each sheet gets its own renderer, so as to avoid loading many small textures
*/
impl SpriteRenderer {
    pub fn new(sprite_sheet_name: String, sprite_record_data: SpriteRecordData) -> SpriteRenderer {
        
        let loaded_image = image::open(sprite_sheet_name).unwrap();
        let pixels = loaded_image.raw_pixels();
        
        let mut texture_index = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_index);
            gl::BindTexture(gl::TEXTURE_2D, texture_index);
            let color_type = match loaded_image.color() {
                image::ColorType::RGB(_) => {
                    gl::RGB
                },
                image::ColorType::RGBA(_) => {
                    gl::RGBA
                },
                image::ColorType::Gray(_) => {
                        gl::RGB
                },
                image::ColorType::GrayA(_) => {
                        gl::RGBA
                },
                image::ColorType::Palette(_) => {
                    gl::RGBA
                }
            };
            gl::TexImage2D(gl::TEXTURE_2D, 0, color_type as i32, loaded_image.dimensions().0 as i32, loaded_image.dimensions().1 as i32, 0, color_type, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const std::os::raw::c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
           
        }

        let frag = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/sprite_shader.vs"
            ))).unwrap(),
            gl::VERTEX_SHADER,
        );

        let vert = Shaders::compile_shader(
            str::from_utf8(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/sprite_shader.fs"
            ))).unwrap(),
            gl::FRAGMENT_SHADER,
        );

        let mut vertex_buffer = 0;
        let mut index_buffer = 0;
        let mut uv_buffer = 0;
        let mut color_buffer = 0;
        let mut sampler_uniform_location : GLint = 0;
        let shader_program_id = Shaders::link_shaders(vert, frag);

        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::GenBuffers(1, &mut index_buffer);
            gl::GenBuffers(1, &mut uv_buffer);
            gl::GenBuffers(1, &mut color_buffer);
            sampler_uniform_location = gl::GetUniformLocation(shader_program_id, CString::new("tex").unwrap().as_ptr());
        }

        SpriteRenderer {
            current_sprite_datas: vec![],
            shader_program: shader_program_id,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture_index: texture_index,
            sampler_uniform_location: sampler_uniform_location,
            uv_buffer: uv_buffer,
            sprites_on_texture: sprite_record_data,
            image: loaded_image
        }
    }

    pub fn render(&mut self, main_renderer: &Renderer) {

        let mut vertex_array: Vec<GLfloat> = vec![];
        let mut index_array: Vec<GLuint> = vec![];
        let mut uv_array: Vec<[GLfloat; 2]> = vec![];

        let mut count = 0;

        for crd in &self.current_sprite_datas {
            let box_data = self.sprites_on_texture.data.get(&crd.sprite_name);
            if !box_data.is_some() {
                continue;
            }

            let box_data = box_data.unwrap();
            vertex_array.extend(
                &[
                    (-0.5 * crd.width) + crd.pos[0],//x
                    (0.5 * crd.height) + crd.pos[1],//y
                    crd.depth,
                    (0.5 * crd.width) + crd.pos[0],
                    (0.5 * crd.height) + crd.pos[1],
                    crd.depth,
                    (-0.5 * crd.width) + crd.pos[0],
                    (-0.5 * crd.height) + crd.pos[1],
                    crd.depth,
                    (0.5 * crd.width) + crd.pos[0],
                    (-0.5 * crd.height) + crd.pos[1],
                    crd.depth
                ],
            );

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
            if crd.reverse_x {
                uv_array.extend(&[[box_data.x + box_data.width, box_data.y], [box_data.x, box_data.y],
                                [box_data.x + box_data.width, box_data.y + box_data.height], [box_data.x, box_data.y + box_data.height]]);
            }
            else {
                uv_array.extend(&[[box_data.x, box_data.y], [box_data.x + box_data.width, box_data.y],
                                [box_data.x, box_data.y + box_data.height], [box_data.x + box_data.width, box_data.y + box_data.height]]);
                
            }
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

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_index);
            gl::Uniform1i(self.sampler_uniform_location, 0);
        }

        //fill buffers
        unsafe {

            //our vertices
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
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            //the uvs for the sprite
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (uv_array.len() * mem::size_of::<GLfloat>() * 2) as GLsizeiptr,
                mem::transmute(uv_array.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1 as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            //the index buffer
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

            //the vertex info
            gl::DisableVertexAttribArray(0);
            //the uv info
            gl::DisableVertexAttribArray(1);
        }
        self.current_sprite_datas.truncate(0);
    }
}
