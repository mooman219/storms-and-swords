use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;
use game::ContentId;
use content::load_content::{EContentType, EContentLoadRequst};
use image::DynamicImage;
use rusttype::{FontCollection, Scale, point, PositionedGlyph, Font};
use frame_timer::FrameTimer;

pub struct ContentManifest {
    pub loaded_images: HashMap<ContentId, DynamicImage>,
    //pub loaded_fonts: HashMap<ContentId, Font>,
    pub loaded_asset_channel: Receiver<EContentType>,
    pub from_render_thread: Receiver<EContentLoadRequst>,
    pub to_render_thread: Sender<EContentType>,
}

impl ContentManifest {
    pub fn new(
        loaded_asset_channel: Receiver<EContentType>,
        from_render_thread: Receiver<EContentLoadRequst>,
        to_render_thread: Sender<EContentType>,
    ) -> ContentManifest {
        ContentManifest {
            loaded_images: HashMap::new(),
            loaded_asset_channel: loaded_asset_channel,
            from_render_thread: from_render_thread,
            to_render_thread: to_render_thread,
        }
    }

    pub fn thread_loop(
        loaded_asset_channel: Receiver<EContentType>,
        from_render_thread: Receiver<EContentLoadRequst>,
        to_render_thread: Sender<EContentType>,
    ) {

        let mut content_manifest: ContentManifest =
            ContentManifest::new(loaded_asset_channel, from_render_thread, to_render_thread);

        let mut frame_check = FrameTimer::new();
        
        loop {
            frame_check.frame_start();
            let possible_new_asset = content_manifest.loaded_asset_channel.try_recv();
            match possible_new_asset {
                Ok(new_asset) => {

                    match new_asset {
                        EContentType::Image(content_id, dynamic_image) => {
                            content_manifest.loaded_images.insert(
                                content_id,
                                dynamic_image,
                            );
                        },
                        EContentType::Font => {
                            
                        },
                        EContentType::NotLoaded => {
                            //just pattern matching, for this part, we should never hit this branch
                        },
                    }
                },
                Err(_) => {
                    //for now do nothing, we likely will not hear anything across this
                },
            };

            let load_request = content_manifest.from_render_thread.try_recv();

            match load_request {
                Ok(content_request) => {
                    match content_request {
                        EContentLoadRequst::Image(content_id) => {
                            if content_manifest.loaded_images.contains_key(&content_id) {
                                let _ =
                                    content_manifest.to_render_thread.send(EContentType::Image(
                                        content_id,
                                        content_manifest.loaded_images.remove(&content_id).unwrap(),
                                    ));
                            } else {
                                let _ = content_manifest.to_render_thread.send(
                                    EContentType::NotLoaded,
                                );
                            }
                        },
                    }
                },
                Err(_) => {},
            }
            frame_check.frame_end();
        }
    }
}
