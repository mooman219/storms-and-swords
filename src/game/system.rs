use game::controller::Controller;
use graphics::renderer::RenderFrame;
use game::playfield_controller::*;
use game::battle_controller::*;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use glutin;
use game::Input;
use frame_timer::FrameTimer;


pub struct MessageBag {
    pub generate_playfield_messages: Vec<GeneratePlayfieldMessage>,
    pub start_game_message: Vec<StartBattleMessage>,
}

impl MessageBag {
    pub fn new() -> MessageBag {
        MessageBag {
            generate_playfield_messages: vec![],
            start_game_message: vec![]
        }
    }
}

pub struct System {
    pub message_bag: MessageBag,
    pub controllers: Vec<Box<Controller>>,
    pub input: Input,

    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: SyncSender<RenderFrame>,
    pub from_render_thread_for_input: Receiver<glutin::KeyboardInput>,

}

impl System {
    pub fn new(to_content_server: Sender<EContentRequestType>,
               from_cotent_server: Receiver<EContentRequestResult>,
               to_render_thread: SyncSender<RenderFrame>,
               from_render_thread_for_input: Receiver<glutin::KeyboardInput>) -> System {
                   
        System {
            message_bag: MessageBag::new(),
            controllers: vec![],
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            from_render_thread_for_input: from_render_thread_for_input,
            input: Input::new()
        }
    }

    pub fn update(mut self) {
        let mut count = 0;
        let mut frame_timer = FrameTimer::new();
        self.controllers.push(Box::new(PlayfieldController::new()));
        self.controllers.push(Box::new(BattleController::new()));
        
        for controller in self.controllers.iter_mut() {
            controller.start();
        }

        loop {
            count+=1;
            frame_timer.frame_start();
            {
                //this must be in its own block as it causes an immtuable borrow of the self varible
                let current_iter = self.from_render_thread_for_input.try_iter();
                for event in current_iter {
                //    println!("{:?}", event);
                    self.input.process_key_event(event);
                }
            }
            if count == 1 {
                self.message_bag.start_game_message.push(StartBattleMessage{});
            }

            for controller in self.controllers.iter_mut() {
                controller.update(&mut self.message_bag);
            }

            let mut render_frame = RenderFrame::new(0, None, None, None);
            
            for controller in self.controllers.iter() {
                controller.add_to_render_frame(&mut render_frame);
            }

            let _ = self.to_render_thread.try_send(render_frame);
            
            self.input.end_of_frame_clean();
            frame_timer.frame_end();
        }
        //send frame over to renderer
    }
}