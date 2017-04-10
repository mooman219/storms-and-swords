use std::sync::mpsc::Receiver;
use graphics::renderable::Renderable;
use graphics::sprite_renderer::{SpriteRenderer, SpriteRenderData};
use std::collections::VecDeque;
use glium::{self, DisplayBuild};
use glium::backend::glutin_backend::GlutinFacade;
use content::load_content::LoadContent;

pub struct RenderFrame {
    pub sprite_renderers: Vec<SpriteRenderData>,
    pub frame_index: u64,//we keep track so we know in what relation we are to the main game loop
}

pub struct RenderThread<'a> {
    display: GlutinFacade,
    from_game_thread: Receiver<RenderFrame>,
    current_frame_index: u64,
    frames: VecDeque<RenderFrame>,
    sprite_renderer: SpriteRenderer,
    load_content: LoadContent<'a>,
}

impl<'a> RenderThread<'a> { 
    pub fn new(from_game_thread: Receiver<RenderFrame>) -> RenderThread<'a> {
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let sprite_renderer = SpriteRenderer::new(&display);
        let load_content = LoadContent::new(&display);

        RenderThread {display: display, 
                      from_game_thread: from_game_thread, 
                      current_frame_index: 0,
                      frames: VecDeque::new(), 
                      sprite_renderer: sprite_renderer,
                      load_content: load_content}
    }

    pub fn render(&mut self) {
        loop {
            let frame = self.frames.front();
            
            if frame.is_none() {
                //a case where the rendere is at the same place the game thread is, just keep spinning until we get another frame
                return;
            }

            let frame  = frame.unwrap();
            //the sprite renderers block
            {
                for srd in frame.sprite_renderers.iter() {
                  //self.sprite_renderer.render(srd, , frame);
                }
            }
        }
    }
}