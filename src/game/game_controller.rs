use game::message_bag::{MessageBag, CurrentState};
use game::battle_controller::{StartBattleMessage};

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
            message_bag.start_battle_message.push(StartBattleMessage{});
            message_bag.next_state = CurrentState::Battle;
        }
    }
}