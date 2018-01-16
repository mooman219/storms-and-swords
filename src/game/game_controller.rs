use std::boxed::Box;
use game::*;
use game::match_controller::MatchController;


pub enum EGameState {
    Battle,
    Map,
    Menu
}

pub struct GameController {
    uid: UID,
    game_state: EGameState
}

impl GameController {
    pub fn new(uid: UID) -> GameController {
        GameController {
            uid,
            game_state: EGameState::Menu

        }
    }

    pub fn change_state(&mut self, game_state: EGameState) {
        match game_state {
            EGameState::Battle => {
                self.game_state = game_state;
            },
            EGameState::Map => {
                self.game_state = game_state;
            },
            EGameState::Menu => {
                self.game_state = game_state;
            }
        }
    }
}

impl EntityController for GameController {
    fn start(&mut self, world: &mut World){
        let mut match_controller = MatchController::new(world.get_uid_for_controller());
        match_controller.start(world);
        world.add_controller_to_world(Box::new(match_controller));
    }
    
    fn update(&self, _world: &World) -> Option<Box<Fn(&mut World, &mut Box<EntityController>)>>{         
        None
    }

    fn get_entity_type(&self) -> EEntityType{
        EEntityType::GameController
    }
    
    fn get_uid(&self) -> UID {
        self.uid
    }
}