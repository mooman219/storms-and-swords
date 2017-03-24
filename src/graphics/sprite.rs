use image::RgbaImage;
use image;
use std::io::Cursor;
use glium::texture::Texture2d;
use glium::{self};
use glium::backend::glutin_backend::GlutinFacade;
use std::path::Path;

pub struct Sprite {
    texture: Texture2d
}

impl Sprite {
    
    pub fn new(image_name: String, display: &GlutinFacade) -> Sprite{

        let image = image::open(&Path::new(&image_name[..])).unwrap().to_rgba();
            
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                                    image.into_raw(), image_dimensions);

        Sprite{texture: Texture2d::new(display, image).unwrap()}
    }

    pub fn get_texture(&self) -> &Texture2d{
        &self.texture
    }
}