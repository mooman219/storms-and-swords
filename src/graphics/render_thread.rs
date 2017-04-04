use std::sync::mpsc::Receiver;
use graphics::renderable::Renderable;
use graphics::sprite_renderer::SpriteRenderData;
use std::collections::VecDeque;

pub struct RenderFrame {
    pub sprite_renderers: Vec<SpriteRenderData>,
    pub frame_index: u64,//we keep track so we know in what relation we are to the main game loop

}

pub struct RenderThread {
    from_game_thread: Receiver<RenderFrame>,
    current_frame_index: u64,
    frames: VecDeque<RenderFrame>
}

impl RenderThread {
    pub fn new(from_game_thread: Receiver<RenderFrame>) -> RenderThread {
        RenderThread {from_game_thread: from_game_thread, current_frame_index: 0,frames: VecDeque::new()}
    }
    pub fn render(&mut self) {
        let frame = self.frames.front();
        if frame.is_none() {
            //a case where the rendere is at the same place the game thread is, just keep spinning until we get another frame
            return;
        }
        let frame  = frame.unwrap();
        //the sprite renderers block
        {
            for srd in frame.sprite_renderers.iter() {
                
            }

        }


    }
}