use game::entity::*;
use game::World;
use graphics::renderer::RenderFrame;
use game::character_model::CharacterModel;
use cgmath::Vector2;
use game::event_system::Event;

pub struct MatchModel {
    uid: UID,
    turn_positions: Vec<UID>,//characters have all the info they need about themsevles
    //such as what team they are one
    current_event_queue: Vec<Box<Event>>,//every update tick the match
    callback_number: u8//every time we trigger an event it gets a callback,
    //we dont adavnce to the next event until all callbacks have been called
}

impl MatchModel {
    pub fn new(world: &mut World) -> MatchModel {
        let mut hold_uid = vec![];

        for i in 0..5 {
            let v = Vector2::new(1, 1 * i - 2);
            let ch_1 = CharacterModel::from_raw_values(v,
                                                    100,
                                                    String::from("hello"),
                                                    10, 
                                                    true,
                                                    0);

            let v_2 = Vector2::new(17, 1 * i - 2);
            let ch_2 = CharacterModel::from_raw_values(v_2,
                                                    100,
                                                    String::from("hello"),
                                                    10,
                                                    false,
                                                    0);
            
            hold_uid.push(world.set_uid_for_entity(Box::new(ch_1)));
            hold_uid.push(world.set_uid_for_entity(Box::new(ch_2)));
        }


        MatchModel {
            uid: 0,
            turn_positions: hold_uid,
            current_event_queue: vec![],
            callback_number: 0
        }
    }
}

impl Entity for MatchModel {
    fn get_entity_type(&self) -> EEntityType{
        EEntityType::Match
    }

    fn get_uid(&self) -> UID{
        self.uid
    }
    fn set_uid(&mut self, uid: UID){
        self.uid = uid;
    }
    fn add_to_render_frame(&self, _render_frame: &mut RenderFrame){

    }
}

pub struct MatchController {
    uid: UID,
    match_model_uid: UID
}
impl MatchController {
    pub fn new(uid: UID) -> MatchController {
        MatchController {
            uid,
            match_model_uid: 0
        }
    }
}

impl EntityController for MatchController {
    fn start(&mut self, world: &mut World){
        let match_model = MatchModel::new(world);
        self.match_model_uid = world.set_uid_for_entity(Box::new(match_model));
    }

    fn update(&self, _world: &World) -> Option<Box<Fn(&mut World, &mut Box<EntityController>)>>{
        return Some(Box::new(|world, entity_controller|{
            //entity_controller.start(world);
            let mc = unsafe { &mut *(entity_controller as *mut Box<EntityController> as *mut MatchController) };
            let mut possible_next_event = None;
            
            {
                let match_model = world.get_mut_entity(mc.match_model_uid);
                match match_model {
                    Some(model) => {
                        let model = entity_to_entity_mut_type!(MatchModel, model);
                        if model.callback_number == 0 {
                            if model.current_event_queue.len() > 0 {
                                let next_event = model.current_event_queue.remove(0);
                                possible_next_event = Some(next_event);
                            }
                        }
                    },
                    None => {
                      //  print!("HAMILTON ERROR: no match model was found, uid: {} might be bad", mc.match_model_uid);
                    }
                }
            }
            

            match possible_next_event {
                Some(next_event) => {
                    next_event.execute(world);
                },
                None => {}
            }
            
        }));
    }

    fn get_entity_type(&self) -> EEntityType{
        EEntityType::Match
    }

    fn get_uid(&self) -> UID{
        self.uid
    }
}