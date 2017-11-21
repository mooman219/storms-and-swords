use std::boxed::Box;
use std::collections::HashMap;

use glutin;
use game::ContentId;
use game::entity::{Entity, UID, EEntityType, EntityController};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use content::load_content::{EContentRequestType, EContentRequestResult};
use graphics::renderer::RenderFrame;
use glutin::VirtualKeyCode;
use game::Input;
use game::tetris_block::{TetrisBlockController};

/*
use game::paddle::{PaddleModel, PaddleController};
use game::ball::{BallModel, BallController};
*/

use frame_timer::FrameTimer;

#[derive(PartialEq, Eq)]
pub enum ELoadContentError {
    ContentMissMatch,
    LoadThreadNoResponce,
}

pub struct World<'a> {
    pub entity_uids: UID,
    pub controller_uid: UID,
    pub to_content_server: Sender<EContentRequestType>,
    pub from_cotent_server: Receiver<EContentRequestResult>,
    pub to_render_thread: SyncSender<RenderFrame>,
    pub from_render_thread_for_input: Receiver<glutin::KeyboardInput>,
    pub test: i32,
    pub input: Input,
    pub entities: HashMap<UID, &'a Entity>,
    pub type_to_uid_list: HashMap<EEntityType, Vec<UID>>,
   // pub key_pressed: [bool; 256],
    pub key_pressed: HashMap<glutin::VirtualKeyCode, bool>,
}

impl<'a> World<'a> {
    pub fn new(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: SyncSender<RenderFrame>,
        from_render_thread_for_input: Receiver<glutin::KeyboardInput>,
    ) -> World<'a> {

        World {
            entity_uids: 1 as u64, //uids start at 1, because we can use 0 as a flag value, a NULL valye
            controller_uid: 1 as u64,
            to_content_server: to_content_server,
            from_cotent_server: from_cotent_server,
            to_render_thread: to_render_thread,
            from_render_thread_for_input: from_render_thread_for_input,
            test: 0 as i32,
            input: Input::new(),
            entities: HashMap::new(),
            type_to_uid_list: HashMap::new(),
            key_pressed: HashMap::new(),
        }
    }

    pub fn update(
        to_content_server: Sender<EContentRequestType>,
        from_cotent_server: Receiver<EContentRequestResult>,
        to_render_thread: SyncSender<RenderFrame>,
        from_render_thread_input: Receiver<glutin::KeyboardInput>,
    ) {

        let world: World = World::new(
            to_content_server,
            from_cotent_server,
            to_render_thread,
            from_render_thread_input,
        );

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
        self.type_to_uid_list.get_mut(&entity_type).unwrap().push(
            entity.get_uid(),
        );
        let entity = unsafe { &mut *Box::into_raw(entity) };
        self.entities.insert(entity.get_uid(), entity);
    }

    pub fn inner_update(mut self) {
        let mut frame_timer = FrameTimer::new();


        let mut frame_count = 0 as u64;
        let controller_uid = self.get_uid_for_controller().clone();
        let mut entity_controllers: HashMap<EEntityType, &mut EntityController> = HashMap::new();
        let new_tetris_block_controller = TetrisBlockController::new(controller_uid);
        let controller_store = unsafe{&mut *Box::into_raw(Box::new(new_tetris_block_controller))};
        entity_controllers.insert(EEntityType::TetrisBlock, controller_store);

        loop {
            frame_timer.frame_start();

            {
                //this must be in its own block as it causes an immtuable borrow of the self varible
                let current_iter = self.from_render_thread_for_input.try_iter();
                for event in current_iter {
                //    println!("{:?}", event);
                    self.input.process_key_event(event);
                }
            }
            
            let mut modify_functions = vec![];
            let mut controllers_type = vec![];

            for controllers in entity_controllers.keys() {
                modify_functions.push(entity_controllers.get(controllers).unwrap().update(&self));//actually genreate the functions
                controllers_type.push(controllers.clone());
            }

            for i in 0..modify_functions.len() {
                modify_functions[i].as_ref().unwrap()(&mut self, *entity_controllers.get_mut(&controllers_type[i]).unwrap());//call all generated functions
            }

            frame_count = frame_count + 1;


            let mut render_frame = RenderFrame::new(frame_count, None, None);

            for ent_uid in &self.entities {
                let ent = ent_uid.1;
                ent.add_to_render_frame(&mut render_frame);
            }

            let _ = self.to_render_thread.try_send(render_frame);
            self.input.end_of_frame_clean();
            frame_timer.frame_end();
        }
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    //entnties (IE those who are creating them) are unable to get uids without also giving the world ownership of that entity
    pub fn set_uid_for_entity(&mut self, mut entity: Box<Entity>) -> UID{
        self.entity_uids +=1;
        entity.set_uid(self.entity_uids);
        self.add_entity(entity);
        return self.entity_uids;
    }

    //calling this for entities is bad, they have two seperate counters for UIDs, and so could clash
    pub fn get_uid_for_controller(&mut self) -> UID {
        self.controller_uid += 1;
        return self.controller_uid;
    }

    pub fn load_content(&self, content: EContentRequestType) -> Result<ContentId, ELoadContentError> {
        let _ = self.to_content_server.send(content);
        let result = self.from_cotent_server.recv();
        match result {
            Ok(return_content) => {
                match return_content {
                    EContentRequestResult::Image(id) => {
                        return Ok(id);
                    },
                }
            },
            Err(_) => {
                return Err(ELoadContentError::LoadThreadNoResponce);
            },
        }
    }
}
