use std::collections::HashMap;
use std::path::Path;

use image;
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;

use graphics::sprite::Sprite;
use game::ContentId;

enum ELoadContentErr {
    ProblemFindingImage(String),
    ProblemConvertingImageToTexture,
}

pub struct LoadContent<'a> {
    sprites: HashMap<ContentId, Sprite>,
    content_count: u64,
    display: &'a GlutinFacade
}

impl<'a> LoadContent<'a> {
    
    fn new (display: &GlutinFacade) -> LoadContent {
        LoadContent{sprites: HashMap::new(), content_count: 0, display: display}
    }

    fn get_sprite(&self, content_id: ContentId) -> Option<&Sprite> {
        self.sprites.get(&content_id)
    }

    fn load_image(&mut self, name: String) -> Result<ProblemFindingImage, ContentId> {
        let image = image::open(&Path::new(&name[..]));
        match image {
            Some(image) => {

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                                        image.into_raw(), image_dimensions);
            
            let tex = Texture2d::new(self.display, image);
            match tex {
                Some(tex) => {
                    let spr = Sprite::new(name, tex, self.display);
                    let content_id = self.content_count += 1;
                    self.sprites.insert(content_id, spr);
                    return Ok(content_id);
                },
                None => {
                    return Err(ELoadContentErr::ProblemConvertingImageToTexture);
                }
            }
            
            },
            None =>{
                return Err(ELoadContentErr::ProblemFindingImage(name));
            }
        }
    }

}