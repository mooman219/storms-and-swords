use std::sync::mpsc::{Receiver, Sender};
use graphics::renderable::Renderable;
use graphics::sprite_renderer::{SpriteRenderer, SpriteRenderData};
use std::collections::VecDeque;
use glium::{self, DisplayBuild};
use glium::backend::glutin_backend::GlutinFacade;
use game::ContentId;
use content::load_content::EContentType;


pub struct RenderFrame {
    pub sprite_renderers: Vec<SpriteRenderData>,
    pub frame_index: u64, //we keep track so we know in what relation we are to the main game loop
}

pub struct RenderThread<'a> {
    display: &'a GlutinFacade,
    from_game_thread: Receiver<RenderFrame>,
    to_content_manifest: Sender<ContentId>,
    from_content_manifest: Receiver<EContentType>,
    current_frame_index: u64,
    frames: VecDeque<RenderFrame>,
    sprite_renderer: SpriteRenderer,
}

impl<'a> RenderThread<'a> {
    pub fn new(display: &GlutinFacade,
               from_game_thread: Receiver<RenderFrame>,
               to_content_manifest: Sender<ContentId>,
               from_content_manifest: Receiver<EContentType>)
               -> RenderThread {

        let sprite_renderer = SpriteRenderer::new(display);

        RenderThread {
            display: display,
            current_frame_index: 0,
            frames: VecDeque::new(),
            sprite_renderer: sprite_renderer,
            from_game_thread: from_game_thread,
            to_content_manifest: to_content_manifest,
            from_content_manifest: from_content_manifest,
        }
    }

    pub fn render(&mut self) {
        loop {
            let frame = self.frames.front();

            if frame.is_none() {
                //a case where the rendere is at the same place the game thread is, just keep spinning until we get another frame
                return;
            }

            let frame = frame.unwrap();
            //the sprite renderers block
            {
                for srd in frame.sprite_renderers.iter() {
                    //self.sprite_renderer.render(srd, , frame);
                }
            }
        }
    }
}
