use std::boxed::Box;
use std::collections::HashMap;

use cgmath::Vector3;

use game::ContentId;
use game::entity::{Entity, UID, EEntityType, EntityController};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::renderer::RenderFrame;
use glutin::VirtualKeyCode;
use game::Input;
use game::paddle::{PaddleModel, PaddleController};
use game::ball::{BallModel, BallController};


use frame_timer::FrameTimer;

#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World<'a> {
    pub uids: UID,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: SyncSender<RenderFrame>,
    pub from_render_thread_for_input: Receiver<VirtualKeyCode>,
    pub test: i32,
    pub input: Input,
    pub entities: HashMap<UID, &'a Entity>,
    pub type_to_uid_list: HashMap<EEntityType, Vec<UID>>,
    pub entity_controllers: HashMap<EEntityType, &'a EntityController>,
}

impl<'a> World<'a> {
    pub fn new(to_content_server: Sender<EContentRequestType>,
               from_cotent_server: Receiver<EContentRequestResult>,
               to_render_thread: SyncSender<RenderFrame>,
               from_render_thread_for_input: Receiver<VirtualKeyCode>)
               -> World<'a> {

        World {
            uids: 1 as u64, //uids start at 1, because we can use 0 as a flag value, a NULL valye
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            from_render_thread_for_input: from_render_thread_for_input,
            test: 0 as i32,
            input: Input::new(),
            entities: HashMap::new(),
            type_to_uid_list: HashMap::new(),
            entity_controllers: HashMap::new(),
        }
    }

    pub fn update(to_content_server: Sender<EContentRequestType>,
                  from_cotent_server: Receiver<EContentRequestResult>,
                  to_render_thread: SyncSender<RenderFrame>,
                  from_render_thread_input: Receiver<VirtualKeyCode>) {

        let world: World = World::new(to_content_server,
                                      from_cotent_server,
                                      to_render_thread,
                                      from_render_thread_input);

        world.inner_update();

    }

    pub fn get_entity(&self, uid: UID) -> Option<&&Entity> {
        self.entities.get(&uid)
    }

    pub fn get_mut_entity(&mut self, uid: UID) -> Option<&mut &'a Entity> {
        self.entities.get_mut(&uid)
    }
    pub fn add_entity(&mut self, entity: Box<Entity>) {
        let entity_type = entity.get_entity_type();
        if !self.type_to_uid_list.contains_key(&entity_type) {
            self.type_to_uid_list.insert(entity_type, Vec::new());
        }
        self.type_to_uid_list.get_mut(&entity_type).unwrap().push(entity.get_uid());
        let entity = unsafe { &mut *Box::into_raw(entity) };
        self.entities.insert(entity.get_uid(), entity);
    }

    pub fn inner_update(mut self) {
        let mut frame_timer = FrameTimer::new();


        let mut frame_count = 0 as u64;
        self.entity_controllers.insert(EEntityType::PADDLE, &PaddleController {});
        self.entity_controllers.insert(EEntityType::BALL, &BallController {});

        let ball_model = BallModel::new(self.get_uid());
        //  self.add_entity(Box::new(ball_model));

        let mut paddle_model_1 = PaddleModel::new(self.get_uid());
        paddle_model_1.set_position(Vector3::new(200.0f32, 0.0f32, 0.0f32));
        paddle_model_1.set_scale(Vector3::new(1.0f32, 1.0f32, 0.0f32));

        let mut paddle_model_2 = PaddleModel::new(self.get_uid());
        paddle_model_2.set_position(Vector3::new(0.0f32, 0.0f32, 0.0f32));
        paddle_model_2.set_scale(Vector3::new(0.25f32, 1.0f32, 0.0f32));

        self.add_entity(Box::new(paddle_model_1));
        self.add_entity(Box::new(paddle_model_2));

        loop {
            frame_timer.frame_start();

            let input_check = self.from_render_thread_for_input.try_recv();

            match input_check {
                Ok(_input_event) => {}
                Err(_e) => {}
            }

            let mut modify_functions = vec![];

            for controllers in &self.entity_controllers {
                modify_functions.push(controllers.1.update(&self));
            }

            for funcs in &modify_functions {
                funcs.as_ref().unwrap()(&mut self);
            }

            frame_count = frame_count + 1;


            let mut render_frame = RenderFrame::new(frame_count, None, None);

            for ent_uid in &self.entities {
                let ent = ent_uid.1;
                ent.add_to_render_frame(&mut render_frame);
            }

            let _ = self.to_render_thread.try_send(render_frame);

            frame_timer.frame_end();
        }
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    pub fn get_uid(&mut self) -> UID {
        self.uids += 1;
        return self.uids;
    }

    pub fn load_content(&self,
                        content: EContentRequestType)
                        -> Result<ContentId, ELoadContentError> {
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
