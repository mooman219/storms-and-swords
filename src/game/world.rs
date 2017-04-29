use std::collections::HashMap;
use game::ContentId;
use game::entity::{Entity, UID};
use std::sync::mpsc::{Receiver, Sender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::render_thread::RenderFrame;

#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World {
    entities: HashMap<UID, Entity>,
    uids: UID,
    to_content_server: Sender<EContentRequestType>,
    from_cotent_server: Receiver<EContentRequestResult>,
    to_render_thread: Sender<RenderFrame>,
}

impl World {
    pub fn new(to_content_server: Sender<EContentRequestType>,
               from_cotent_server: Receiver<EContentRequestResult>,
               to_render_thread: Sender<RenderFrame>)
               -> World {

        World {
            entities: HashMap::<UID, Entity>::new(),
            uids: 0.0 as i64,
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
        }
    }

    pub fn get_uid(&mut self) -> UID {
        self.uids += 1;
        return self.uids.clone();
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.get_uid().clone(), entity);
    }

    pub fn get_entity(&self, uid: UID) -> Option<&Entity> {
        self.entities.get(&uid)
    }

    pub fn load_content(&self,
                        content: EContentRequestType)
                        -> Result<ContentId, ELoadContentError> {
        let _ = self.to_content_server.send(content);
        let result = self.from_cotent_server.recv();
        match result {
            Ok(return_content) => {
                match return_content {
                    EContentRequestResult::StaticSprite(id) => {
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
