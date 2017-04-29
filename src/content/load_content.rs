use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

use image;
use image::DynamicImage;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;
use threadpool::ThreadPool;
use game::ContentId;

#[derive(Clone)]
enum ELoadContentErr {
    ProblemFindingImage(String),
    ProblemConvertingImageToTexture,
}

#[derive(Clone)]
pub enum EContentRequestType {
    StaticSprite(String),
}

#[derive(Copy, Clone)]
pub enum EContentRequestResult {
    StaticSprite(ContentId),
}

#[derive(Clone)]
pub enum EContentType {
    StaticSprite(ContentId, DynamicImage),
}

pub struct LoadContent {
    content_count: u64,
    from_game_thread: Receiver<EContentRequestType>,
    to_player_thread: Sender<EContentRequestResult>,
    to_content_manifest: Sender<EContentType>,
    thread_pool: ThreadPool,
}

impl LoadContent {
    pub fn new(from_game_thread: Receiver<EContentRequestType>,
               to_player_thread: Sender<EContentRequestResult>,
               to_content_manifest: Sender<EContentType>)
               -> LoadContent {

        LoadContent {
            content_count: 0,
            from_game_thread: from_game_thread,
            to_player_thread: to_player_thread,
            to_content_manifest: to_content_manifest,
            thread_pool: ThreadPool::new(100),
        }
    }

    fn thread_loop(mut content_loader: LoadContent) {

        loop {
            content_loader.inner_thread_loop();
        }

    }

    pub fn inner_thread_loop(&mut self) {
        let result = self.from_game_thread.try_recv();
        match result {
            Ok(content_to_load) => {

                match content_to_load {
                    EContentRequestType::StaticSprite(sprite_to_load) => {

                        let use_content_id = self.content_count.clone();
                        self.content_count = self.content_count + 1;

                        let to_content_manifest_for_thread = self.to_content_manifest.clone();

                        let clo = move || {
                            load_image(sprite_to_load,
                                       use_content_id,
                                       to_content_manifest_for_thread);
                        };


                        self.thread_pool.execute(clo);

                        //self.load_image(sprite_to_load);
                    }
                }
            }
            Err(_) => {}
        }
    }
}

pub fn load_image(name: String, content_id: ContentId, to_content_manifest: Sender<EContentType>) {
    let load_image = image::open(&Path::new(&name[..]));
    match load_image {
        Ok(use_image) => {

            let _ = to_content_manifest.send(EContentType::StaticSprite(content_id, use_image));

        }
        Err(_) => {}

    }
}
