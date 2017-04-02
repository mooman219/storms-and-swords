use std::collections::HashMap;
use game::entity::{Entity, UID};

pub struct World {
    entities: HashMap<UID, Entity>,
    uids: UID,
}

impl World {
    pub fn new() -> World {
        World{entities: HashMap::<UID, Entity>::new(), uids: 0.0 as i64}
    }

    pub fn get_uid(&mut self) -> UID {
        self.uids+=1;
        return self.uids.clone();
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.get_uid().clone(), entity);
    }

    pub fn get_entity(&self, uid: UID) -> Option<&Entity> { 
        self.entities.get(&uid)
    }
 }