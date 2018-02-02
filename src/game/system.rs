use game::controller::Controller;

use game::game_controller::GameController;

use game::battle_controller::{BattleController, BattleControllerState};
use game::playfield_controller::PlayfieldController;
use game::main_menu_controller::MainMenuController;
use graphics::renderer::RenderFrame;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use frame_timer::FrameTimer;
use game::input::InputMessage;
use game::message_bag::{MessageBag, CurrentState};

pub struct System {
    pub message_bag: MessageBag,
    pub controllers: Vec<Box<Controller>>,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: SyncSender<RenderFrame>,
    pub from_render_thread_for_input: Receiver<InputMessage>,
    pub game_controller: GameController,
    pub battle_controller: BattleController,
    pub playfield_controller: PlayfieldController,
    pub main_menu_controller: MainMenuController,
    pub current_state: CurrentState
}

impl System {
    pub fn new(to_content_server: Sender<EContentRequestType>,
               from_cotent_server: Receiver<EContentRequestResult>,
               to_render_thread: SyncSender<RenderFrame>,
               from_render_thread_for_input: Receiver<InputMessage>) -> System {
                   
        System {
            message_bag: MessageBag::new(),
            controllers: vec![],
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            from_render_thread_for_input: from_render_thread_for_input,
            battle_controller: BattleController::new(),
            playfield_controller: PlayfieldController::new(),
            main_menu_controller: MainMenuController::new(),
            game_controller: GameController::new(),
            current_state: CurrentState::MainMenu
        }
    }

    

    pub fn update(mut self) {
        let mut count = 0;
        let mut frame_timer = FrameTimer::new();
        let mut message_bag = MessageBag::new();


        loop {
            frame_timer.frame_start();
            
            {
                //this must be in its own block as it causes an immtuable borrow of the self varible
                let current_iter = self.from_render_thread_for_input.try_iter();
                for event in current_iter {
                    self.message_bag.input.process_event(event);
                }
            }

            match self.current_state {        
                CurrentState::MainMenu => {
                    self.main_menu(&mut message_bag);
                },
                CurrentState::Battle => {
                    self.battle(&mut message_bag);
                }
            }


            self.current_state = message_bag.next_state;
            frame_timer.frame_end();
        }
        /*
        
        self.controllers.push(Box::new(GameController::new()));
        self.message_bag.start_game_message.push(StartGameMessage{});

        for controller in self.controllers.iter_mut() {
            controller.start();
        }

        loop {
            count+=1;
            frame_timer.frame_start();
            
            if count == 1 {
                self.message_bag.start_game_message.push(StartGameMessage{});
            }

            for controller in self.controllers.iter_mut() {
                controller.update(&mut self.message_bag);
            }

            let mut render_frame = RenderFrame::new(0, None, None, None);
            
            for controller in self.controllers.iter() {
                controller.add_to_render_frame(&mut render_frame);
            }

            let _ = self.to_render_thread.try_send(render_frame);
            self.message_bag.input.end_of_frame_clean();
            frame_timer.frame_end();
        }
        */
    }

    fn main_menu(&mut self, message_bag: &mut MessageBag) {
        //TODO: Find a better solution for rendering    
        let mut render_frame = RenderFrame::new(0, None, None, None);
        self.main_menu_controller.check_for_input(message_bag);
        self.game_controller.check_for_battle_start(message_bag);
        self.main_menu_controller.render_main_menu(&mut render_frame);
        let _ = self.to_render_thread.try_send(render_frame);
    
    }

    fn battle(&mut self, message_bag: &mut MessageBag) {

        let mut render_frame = RenderFrame::new(0, None, None, None);

        match self.battle_controller.current_battle_controller_state {
            BattleControllerState::Setup => {
                self.playfield_controller.check_for_new_playfield_message(message_bag);
                self.battle_controller.battle_setup(message_bag);
            },
            BattleControllerState::InBattle => {
                self.playfield_controller.set_active_tile(message_bag);
                self.playfield_controller.render_playfield(&mut render_frame);
                self.battle_controller.render_characters(&mut render_frame);
            }
        }

        let _ = self.to_render_thread.try_send(render_frame);
    }
}