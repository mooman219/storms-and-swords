use graphics::renderer::RenderFrame;
use graphics::sprite_renderer::SpriteRenderData;
use game::game_controller::StartGameMessage;
use game::message_bag::MessageBag;
use graphics::renderer::{BASE_SCREEN_HEIGHT, BASE_SCREEN_WIDTH, SCREEN_SCALE};
use glutin::VirtualKeyCode;

pub struct MainMenuController {
    
}


impl MainMenuController {
    pub fn new() -> MainMenuController {
        MainMenuController {

        }
    }

    pub fn check_for_input(&mut self, message_bag: &mut MessageBag) {
        if message_bag.input.on_key_pressed(VirtualKeyCode::Space) {
            message_bag.start_game_message.push(StartGameMessage{});
        }
    }

    pub fn render_main_menu(&self, render_frame: &mut RenderFrame) {
        let srd = SpriteRenderData{
            pos: [0.0, 0.0],
            sprite_name : String::from("main_menu_texture_1"),
            height: BASE_SCREEN_HEIGHT * SCREEN_SCALE,
            width: BASE_SCREEN_WIDTH * SCREEN_SCALE,
            depth: 0.0,
            reverse_x: false,
        };

        if render_frame.static_sprites.is_none() {
            render_frame.static_sprites = Some(vec![]);
        }

        render_frame.static_sprites.as_mut().unwrap().push(srd);
    }
}