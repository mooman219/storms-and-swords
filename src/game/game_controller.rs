use game::controller::Controller;
use game::system::*;
use game::battle_controller::*;
use game::playfield_controller::*;
use graphics::renderer::RenderFrame;

pub struct StartGameMessage {

}

pub struct GameController {

}

impl GameController {
    pub fn new() -> GameController {
        GameController{}
    }
}

impl Controller for GameController {
    fn start(&mut self) {

    }

    fn update(&mut self, message_bag: &mut MessageBag) {
        if message_bag.start_game_message.len() > 0 {
            message_bag.start_game_message.drain(..);
            message_bag.new_controllers.push(Box::new(PlayfieldController::new()));
            message_bag.new_controllers.push(Box::new(BattleController::new()));
            message_bag.generate_playfield_messages.push(GeneratePlayfieldMessage{});
            message_bag.start_battle_message.push(StartBattleMessage{});
        }
    }

    fn add_to_render_frame(&self, _render_frame: &mut RenderFrame) {

    }
}