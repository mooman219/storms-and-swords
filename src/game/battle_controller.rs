use game::message_bag::MessageBag;
use game::playfield_controller::PlayfieldController;
use game::in_battle_character::InBattleCharacterModel;
use graphics::renderer::RenderFrame;
use cgmath::Vector2;

#[derive(Clone, Copy)]
pub enum BattleControllerState {
    Setup,
    InBattle
}

pub struct StartBattleMessage {}

pub struct BattleController {
    in_battle_characters: Vec<InBattleCharacterModel>,
    current_battle_controller_state: BattleControllerState,
    playfield_controller: PlayfieldController,
}

impl BattleController {
    pub fn new() -> BattleController {
        BattleController {
            in_battle_characters: vec![],
            current_battle_controller_state: BattleControllerState::Setup,
            playfield_controller:  PlayfieldController::new(),
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

    //this is all that needs for a battle to be played out from start to finish
    //all it needs is a BattleStartMessage with all the correct setup paramaters(will be added as they become defined)
    pub fn battle_main_loop(&mut self, message_bag: &mut MessageBag) {
        match self.current_battle_controller_state {
            BattleControllerState::Setup => {
                self.battle_setup(message_bag);
            },
            BattleControllerState::InBattle => {
                self.battle_idle(message_bag);
            }
        }
    }

    pub fn battle_setup(&mut self, message_bag: &mut MessageBag) {
        if message_bag.start_battle_message.len() > 0 {
            message_bag.start_battle_message.drain(..);
            self.generate_troops();
            self.playfield_controller.new_playfield();
            self.current_battle_controller_state = BattleControllerState::InBattle;
        }
    }

    pub fn battle_idle(&mut self, message_bag: &mut MessageBag) {
        self.playfield_controller.set_active_tile(message_bag);
    }

    pub fn render_characters(&mut self, render_frame: &mut RenderFrame) {
        for character in self.in_battle_characters.iter() {
            character.add_to_render_frame(render_frame);
        }
        self.playfield_controller.render_playfield(render_frame);
    }

    pub fn get_current_battle_state(&self) -> BattleControllerState{
        self.current_battle_controller_state.clone()
    }

    pub fn set_battle_state(&mut self, new_state: BattleControllerState) {
        self.current_battle_controller_state = new_state;
    }
}