use game::entity::{Entity, UID, EEntityType, EntityController};
use game::world::World;
use graphics::renderer::RenderFrame;
use graphics::square_renderer::SquareRenderData;
use game::ui_components::screen_blockers::ScreenBlockers;
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

    fn start(&mut self, world: &mut World) {

    }

    fn update(&self, _world: &World) -> Option<Box<Fn(&mut World, &mut EntityController)>> {
        return Some(Box::new(
            |inner_world: &mut World, controller: &mut EntityController| {
                let uic = unsafe {&mut *(controller as *mut EntityController as *mut UIController)};
                
                if !uic.has_spawned_blockers {
                    uic.has_spawned_blockers = true;
                    let mut sb_1 = ScreenBlockers::new(064);
                    sb_1.set_pos(Vector3::<f32>::new(1450f32, 0.0f32, 0.0f32));

                    let mut sb_2 = ScreenBlockers::new(064);
                    sb_2.set_pos(Vector3::<f32>::new(-1550.0f32, 0.0f32, 0.0f32));

                    let _ = inner_world.set_uid_for_entity(Box::new(sb_1));
                    let _ = inner_world.set_uid_for_entity(Box::new(sb_2));
                    

                }
            })
        );
    }


    fn get_entity_type(&self) -> EEntityType {
        EEntityType::UI
    }

    fn get_uid(&self) -> UID{
       self.uid
    }
}

