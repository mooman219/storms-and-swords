use game::controller::Controller;
use game::system::MessageBag;
use game::in_battle_character::InBattleCharacterModel;
use graphics::renderer::RenderFrame;
use cgmath::Vector2;

pub struct StartBattleMessage {}

pub struct BattleController {
    in_battle_characters: Vec<InBattleCharacterModel>,
    count: i32
}

impl BattleController {
    pub fn new() -> BattleController {
        BattleController {
            in_battle_characters: vec![],
            count: 0
        }
    }
}

impl Controller for BattleController {
    fn start(&mut self) {
        
    }

    fn update(&mut self, message_bag: &mut MessageBag){
        if message_bag.start_game_message.len() > 0 {
            message_bag.start_game_message.drain(..);
            for i in 0..5 {
                let v = Vector2::new(1, 1 * i - 2);
                let ch_1 = InBattleCharacterModel::from_raw_values(v,
                                                                100,
                                                                String::from("hello"),
                                                                10, 
                                                                true);

                let v_2 = Vector2::new(17, 1 * i - 2);
                let ch_2 = InBattleCharacterModel::from_raw_values(v_2,
                                                                100,
                                                                String::from("hello"),
                                                                10,
                                                                false);
                self.in_battle_characters.push(ch_1);
                self.in_battle_characters.push(ch_2);  
          }
        }
        self.count += 1;
        
        if self.count % 60 == 0 {
            self.in_battle_characters[0].set_pos(Vector2::new(5, 5));
        }
    }

    fn add_to_render_frame(&self, render_frame: &mut RenderFrame){
        for character in self.in_battle_characters.iter() {
            character.add_to_render_frame(render_frame);
        }
    }
}