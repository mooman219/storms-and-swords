use game::ContentId;
use game::entity::{Entity, UID};
use std::sync::mpsc::{Receiver, Sender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::render_thread::RenderFrame;
use std::boxed::Box;
use std::collections::HashMap;
use game::Input;
use game::paddle::Paddle;

#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World {
    pub uids: UID,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: Sender<RenderFrame>,
    pub test : i32,
    pub input: Input,
    pub pad: Paddle,

}

impl World {
    pub fn new(to_content_server: Sender<EContentRequestType>,
               from_cotent_server: Receiver<EContentRequestResult>,
               to_render_thread: Sender<RenderFrame>,
               pad: Paddle)
               -> World {

        World {
            uids: 0.0 as i64,
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            test: 0 as i32,
            input: Input{},
            pad: pad
        }
    }


    pub fn update(to_content_server: Sender<EContentRequestType>,
                  from_cotent_server: Receiver<EContentRequestResult>,
                  to_render_thread: Sender<RenderFrame>) {
        
   //     let world : World  = World::new(to_content_server, from_cotent_server, to_render_thread);
     //   world.inner_update();
        
    }

    pub fn inner_update(mut self) {
            loop {

                //the first thing we do is spawn all the new entities that the previous frame asked us to create
                //this includes setting up all the content request to the loading thread

                //we then update all the current entitites
                let mut ents = Vec::<Box<Entity>>::new();
                let len = ents.len();

                for i in 0..len {
                    ents[i].update(&mut self);
                }

                //the last thing we do is go through all entitites and get those that want to be draw that thread and send that information off to the render thread
            }
    }

    pub fn get_uid(&mut self) -> UID {
        self.uids += 1;
        return self.uids.clone();
    }

    pub fn load_content(&self,
                        content: EContentRequestType)
                        -> Result<ContentId, ELoadContentError> {
        let _ = self.to_content_server.send(content);
        let result = self.from_cotent_server.recv();
        match result {
            Ok(return_content) => {
                match return_content {
                    EContentRequestResult::Image(id) => {
                        return Ok(id);
                    }
                }
            }
            Err(_) => {
                return Err(ELoadContentError::LoadThreadNoResponce);
            }
        }
    }
}
