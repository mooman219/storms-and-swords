use std::boxed::Box;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::thread::sleep;

use cgmath::Vector3;

use game::ContentId;
use game::entity::{Entity, UID};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::render_thread::RenderFrame;
use glutin::VirtualKeyCode;
use game::Input;
use game::paddle::{PaddleModel, PaddleController};


use frame_timer::FrameTimer;

#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World {
    pub uids: UID,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: SyncSender<RenderFrame>,
    pub from_render_thread_for_input: Receiver<VirtualKeyCode>,
    pub test: i32,
    pub input: Input,
    pub left_paddle: Option<PaddleModel>,
    pub right_paddle: Option<PaddleModel>
}

impl World {
    pub fn new(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: SyncSender<RenderFrame>,
        from_render_thread_for_input: Receiver<VirtualKeyCode>,
    ) -> World {

        World {
            uids: 0 as i64,
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            from_render_thread_for_input: from_render_thread_for_input,
            test: 0 as i32,
            input: Input::new(),
            left_paddle: None,
           right_paddle: None
        }
    }

    pub fn update(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: SyncSender<RenderFrame>,
        from_render_thread_input: Receiver<VirtualKeyCode>,
    ) {

        let mut world: World = World::new(
            to_content_server,
            from_cotent_server,
            to_render_thread,
            from_render_thread_input,
        );

        let mut left_paddle_model = PaddleModel::new(world.get_uid());
        let mut right_paddle_model = PaddleModel::new(world.get_uid());

        left_paddle_model.set_position(Vector3::new(-0.8f32, -0.0f32, 0.0f32));
        right_paddle_model.set_position(Vector3::new(0.8f32, -0.0f32, 0.0f32));

        left_paddle_model.set_scale(Vector3::new(0.25f32, 1.0f32, 1.0f32));
        right_paddle_model.set_scale(Vector3::new(0.25f32, 1.0f32, 1.0f32));

        world.set_left_paddle(left_paddle_model);
        world.set_right_paddle(right_paddle_model);

        world.inner_update();

    }

    pub fn set_left_paddle(&mut self, paddle_model: PaddleModel) {
        self.left_paddle = Some(paddle_model);
    }

    pub fn set_right_paddle(&mut self, paddle_model: PaddleModel) {
        self.right_paddle = Some(paddle_model);
    }

    pub fn inner_update(mut self) {
        let mut frame_timer = FrameTimer::new();


        let mut frame_count = 0 as u64;
        let mut paddle_vec;

        loop {
            paddle_vec = vec![];
            frame_timer.frame_start();

            let input_check = self.from_render_thread_for_input.try_recv();

            match input_check {
                Ok(_input_event) => {}
                Err(_e) => {}
            }

            frame_count = frame_count + 1;
            self.left_paddle.as_mut().unwrap().move_pos_x(0.0000001f32);
            
            paddle_vec.push(self.left_paddle.as_ref().unwrap().get_box_render_data());
            paddle_vec.push(self.right_paddle.as_ref().unwrap().get_box_render_data());

            let frame_data = RenderFrame::new(frame_count, Some(paddle_vec), None);
            let _ = self.to_render_thread.try_send(frame_data);

            frame_timer.frame_end();
        }
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    pub fn get_uid(&mut self) -> UID {
        self.uids += 1;
        return self.uids.clone();
    }

    pub fn load_content(
        &self,
        content: EContentRequestType,
    ) -> Result<ContentId, ELoadContentError> {
        let _ = self.to_content_server.send(content);
        let result = self.from_cotent_server.recv();
        match result {
            Ok(return_content) => {
                match return_content {
                    EContentRequestResult::Image(id) => {
                        return Ok(id);
                    }
                }
            }
            Err(_) => {
                return Err(ELoadContentError::LoadThreadNoResponce);
            }
        }
    }
}
