use game::ContentId;
use game::entity::{Entity, UID};
use std::sync::mpsc::{Receiver, Sender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::render_thread::RenderFrame;
use graphics::static_sprite_trait::StaticSprite;
use std::boxed::Box;
use std::collections::HashMap;
use game::Input;
use game::paddle::{PaddleModel, PaddleController};


#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World {
    pub uids: UID,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: Sender<RenderFrame>,
    pub test: i32,
    pub input: Input,
    pub left_paddle: Option<PaddleModel>,
   // pub right_paddle: PaddleModel
}

impl World {
    pub fn new(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: Sender<RenderFrame>,
    ) -> World {

        World {
            uids: 0 as i64,
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            test: 0 as i32,
            input: Input::new(),
            left_paddle: None,
          //  right_paddle: right_paddle
        }
    }

    pub fn update(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: Sender<RenderFrame>,
    ) {

        let mut world: World = World::new(to_content_server, from_cotent_server, to_render_thread);
        let content_id_result =
            world.load_content(EContentRequestType::Image("foo.png".to_string()));
        match content_id_result {
            Ok(content_id) => {

                let paddle_model = PaddleModel::new(world.get_uid(), content_id);
                world.set_left_paddle(paddle_model);
                world.inner_update();
            }
            Err(e) => {}
        }

    }
    pub fn set_left_paddle(&mut self, paddle_model: PaddleModel) {
        self.left_paddle = Some(paddle_model);
    }

    pub fn inner_update(mut self) {
        //the controller for both of the paddles
        let paddle_controller = PaddleController::new();

        let mut frame_count = 0 as u64;
        return;
        loop {
            //first we poll for input
                //then we act on that input
                //then we render what the new state of world is
                /*
                let changes = paddle_controller.update(&self);
                match changes {
                    Some(func) => {
                        func(&mut self);
                    },
                    None => {

                    }
                }
*/
            let paddle = self.left_paddle.as_ref().unwrap();

            let data = paddle.generate_sprite_render_data().clone();

            let mut render_frame = RenderFrame::new(frame_count.clone());
            render_frame.sprite_renderers.push(data);
            self.to_render_thread.send(render_frame);

            frame_count = frame_count + 1;
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
