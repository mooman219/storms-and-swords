use game::entity::{Entity, UID, EEntityType, EntityController};
use game::world::World;
use graphics::renderer::RenderFrame;
//use graphics::renderer::RenderFrame;
//use graphics::square_renderer::SquareRenderData;
use cgmath::Vector3;


//I want all ui components to be a subset of entities
//UI is a niche of entities more then just game ones, and as such I think
//it is worth it for them to have there own set of required functions on top of the
//ones that entity has
pub trait UIEntity : Entity {
    fn get_depth() -> usize;
}


pub struct UIController {
    pub uid: UID,
    pub ui_component_list: Vec<UID>,
    pub has_spawned_blockers: bool,

}

impl UIController {
    pub fn new(uid: UID) -> UIController {

        UIController {
            ui_component_list: vec![],
            uid,
            has_spawned_blockers: false
        }
    }
}

impl EntityController for UIController {

    fn start(&mut self, _world: &mut World) {

    }

    fn update(&self, _world: &World) -> Option<Box<Fn(&mut World, &mut Box<EntityController>)>> {
        None
    }


    fn get_entity_type(&self) -> EEntityType {
        EEntityType::UI
    }

    fn get_uid(&self) -> UID{
       self.uid
    }
}

