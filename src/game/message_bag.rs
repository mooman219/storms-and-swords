
use game::playfield_controller::*;
use game::battle_controller::*;
use game::Input;
use game::game_controller::*;

#[derive(PartialEq, Copy, Clone)]
pub enum CurrentState {
    MainMenu,
    Battle
}

pub struct MessageBag {
    pub start_game_message: Vec<StartGameMessage>,
    pub start_battle_message: Vec<StartBattleMessage>,
    pub generate_playfield_messages: Vec<GeneratePlayfieldMessage>,
    pub next_state: CurrentState,
    pub input: Input,
}

impl MessageBag {
    pub fn new() -> MessageBag {
        MessageBag {
            generate_playfield_messages: vec![],
            start_game_message: vec![],
            start_battle_message: vec![],
            input: Input::new(),
            next_state: CurrentState::MainMenu
        }
    }
}
