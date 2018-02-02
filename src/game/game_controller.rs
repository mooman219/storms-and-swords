use game::playfield_controller::*;
use game::message_bag::{MessageBag, CurrentState};

pub struct StartGameMessage {

}

pub struct GameController {

}

impl GameController {
    pub fn new() -> GameController {
        GameController{}
    }

    pub fn check_for_battle_start(&mut self, message_bag: &mut MessageBag) {
        if message_bag.start_game_message.len() > 0 {
            message_bag.start_game_message.drain(..);
            message_bag.generate_playfield_messages.push(GeneratePlayfieldMessage{});
            message_bag.next_state = CurrentState::Battle;
        }
    }
}