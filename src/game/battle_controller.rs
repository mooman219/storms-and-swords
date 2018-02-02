use game::message_bag::MessageBag;
use game::in_battle_character::InBattleCharacterModel;
use graphics::renderer::RenderFrame;
use cgmath::Vector2;

pub enum BattleControllerState {
    Setup,
    InBattle
}

pub struct StartBattleMessage {}

pub struct BattleController {
    in_battle_characters: Vec<InBattleCharacterModel>,
    pub current_battle_controller_state: BattleControllerState
}

impl BattleController {
    pub fn new() -> BattleController {
        BattleController {
            in_battle_characters: vec![],
            current_battle_controller_state: BattleControllerState::Setup
        }
    }

    pub fn generate_troops(&mut self) {
        for i in 5..10 {
                let v = Vector2::new(1, 1 * i);
                let ch_1 = InBattleCharacterModel::from_raw_values(v,
                                                                100,
                                                                String::from("hello"),
                                                                10, 
                                                                true);

                let v_2 = Vector2::new(17, 1 * i);
                let ch_2 = InBattleCharacterModel::from_raw_values(v_2,
                                                                100,
                                                                String::from("hello"),
                                                                10,
                                                                false);
                self.in_battle_characters.push(ch_1);
                self.in_battle_characters.push(ch_2);  
          }
    }

    pub fn battle_setup(&mut self, message_bag: &mut MessageBag) {
        if message_bag.start_battle_message.len() > 0 {
            message_bag.start_battle_message.drain(..);
            self.generate_troops();
        }
    }

    pub fn render_characters(&mut self, render_frame: &mut RenderFrame) {
        for character in self.in_battle_characters.iter() {
            character.add_to_render_frame(render_frame);
        }
    }
}