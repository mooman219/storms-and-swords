use std::sync::mpsc::{Receiver, Sender};
use std::cell::RefCell;
use std::collections::HashMap;
use graphics::sprite::Sprite;
use game::ContentId;
use glium::texture::Texture2d;
use content::load_content::{EContentType, EContentRequestType};
use glium::backend::glutin_backend::GlutinFacade;
use glium;

pub struct ContentManifest {
    pub sprites: HashMap<ContentId, Sprite>,
    pub loaded_asset_channel: Receiver<EContentType>,
    pub from_render_thread: Receiver<ContentId>,
    pub to_render_thread: Sender<EContentType>, 
    pub display: RefCell<GlutinFacade>,
}

impl ContentManifest {
    pub fn new(display: RefCell<GlutinFacade>,
               loaded_asset_channel: Receiver<EContentType>,
               from_render_thread: Receiver<ContentId>,
               to_render_thread: Sender<EContentType>)
               -> ContentManifest {
        ContentManifest {
            sprites: HashMap::new(),
            loaded_asset_channel: loaded_asset_channel,
            from_render_thread: from_render_thread,
            to_render_thread: to_render_thread,
            display: display,
        }
    }

    pub fn thread_loop(display: RefCell<GlutinFacade>,
                       loaded_asset_channel: Receiver<EContentType>,
                       from_render_thread: Receiver<ContentId>,
                       to_render_thread: Sender<EContentType>) {

        let mut content_manifest: ContentManifest = ContentManifest::new(display,
                                                                         loaded_asset_channel,
                                                                         from_render_thread,
                                                                         to_render_thread);

        loop {
            let possible_new_asset = content_manifest.loaded_asset_channel.try_recv();
            match possible_new_asset {
                Ok(new_asset) => {

                    match new_asset {
                        EContentType::StaticSprite(contend_id, dynamic_image) => {
                            /*
                            let image_dimensions = dynamic_image.to_rgba().dimensions();
                            let loaded_image = glium::texture::RawImage2d::from_raw_rgba_reversed(dynamic_image.to_rgba().into_raw(), image_dimensions);
                            let tex = Texture2d::new(content_manifest.display, loaded_image);
                            match tex {
                                Ok(tex) => {
                                    content_manifest.sprites.insert(contend_id, Sprite::new("an_image".to_string(), tex, content_manifest.display);
                                },
                                Err(_) => {

                                }
                            }
                            */
                        }
                    }
                }
                Err(_) => {
                    //for now do nothing, we likely will not hear anything across this
                }
            };


        }
    }
}
