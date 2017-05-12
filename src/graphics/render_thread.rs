use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use graphics::sprite_renderer::{SpriteRenderer, SpriteRenderData};
use glium::{self, DisplayBuild};
use glium::backend::glutin_backend::GlutinFacade;
use game::ContentId;
use content::load_content::{EContentType, EContentLoadRequst};
use glium::texture::Texture2d;
use graphics::sprite::Sprite;

#[derive(Clone)]
pub struct RenderFrame {
    pub sprite_renderers: Vec<SpriteRenderData>,
    pub frame_index: u64, //we keep track so we know in what relation we are to the main game loop
}

pub struct RenderThread {
    display: GlutinFacade,
    from_game_thread: Receiver<RenderFrame>,
    to_content_manifest: Sender<EContentLoadRequst>,
    from_content_manifest: Receiver<EContentType>,
    _current_frame_index: u64,
    sprite_renderer: SpriteRenderer,
    sprites: HashMap<ContentId, Sprite>
}

impl RenderThread {
    pub fn new(from_game_thread: Receiver<RenderFrame>,
               to_content_manifest: Sender<EContentLoadRequst>,
               from_content_manifest: Receiver<EContentType>)
               -> RenderThread {
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let sprite_renderer = SpriteRenderer::new(&display);

        RenderThread {
            display: display,
            _current_frame_index: 0,
            sprite_renderer: sprite_renderer,
            from_game_thread: from_game_thread,
            to_content_manifest: to_content_manifest,
            from_content_manifest: from_content_manifest,
            sprites: HashMap::new()
        }
    }

    pub fn query_content_manifest_for_sprite(&mut self, content_id: ContentId) -> bool {
        if self.sprites.contains_key(&content_id) {
           true
        }
        else {
            let _ = self.to_content_manifest.send(EContentLoadRequst::Image(content_id));
            let value = self.from_content_manifest.recv().unwrap();
            match value {
                EContentType::Image(id, dy_image) => {
                    let image_dimensions = dy_image.to_rgba().dimensions();
                    let loaded_image = glium::texture::RawImage2d::from_raw_rgba_reversed(dy_image.to_rgba().into_raw(), image_dimensions);
                    let tex = Texture2d::new(&self.display, loaded_image).unwrap();
                    let spr = Sprite::new("Sprite".to_string(), tex, &self.display);
                    self.sprites.insert(id, spr);
                    true
                },
                EContentType::NotLoaded => {
                    false
                }
            }
        }

    }

    pub fn get_sprite(&self, content_id: ContentId) -> Option<&Sprite> {
        if self.sprites.contains_key(&content_id) {
            return self.sprites.get(&content_id);
        }
        None
    }

    pub fn thread_loop(from_game_thread: Receiver<RenderFrame>,
                       to_content_manifest: Sender<EContentLoadRequst>,
                       from_content_manifest: Receiver<EContentType>) {

        let mut rend = RenderThread::new(from_game_thread, to_content_manifest, from_content_manifest);

        rend.render();
    }

    pub fn render(&mut self) {
        loop {
            //the first thing we do is grab the current frame
            let frame_data = self.from_game_thread.try_recv();

            if frame_data.is_err() {
                //a case where the renderer is at the same place the game thread is, just keep spinning until we get another frame
                continue;
            }
            let frame_data = frame_data.unwrap();
            let mut frame = self.display.draw();
            //the sprite renderers block
            {
                for srd in frame_data.sprite_renderers.iter() {
                    //this is split over two function calls so as to avoid problems with the borrow checker
                    let result = {
                        self.query_content_manifest_for_sprite(srd.sprite)
                    };

                    if result == true {
                        let use_sprite = self.get_sprite(srd.sprite);
                        match use_sprite {
                            Some(use_sprite) => {
                                self.sprite_renderer.render(srd, use_sprite, &mut frame);
                            },
                            None => {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
