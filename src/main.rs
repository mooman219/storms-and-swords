#![feature(asm, const_fn, const_size_of)]

extern crate glutin;

extern crate cgmath;
extern crate frame_timer;
extern crate gl;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate threadpool;


#[macro_use]
mod macros;

pub mod game;
pub mod graphics;
pub mod math;
pub mod physics;
pub mod content;


use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::mpsc;
use std::thread;

use content::load_content::{EContentLoadRequst, EContentRequestResult, EContentRequestType, EContentType};
use content::{ContentManifest, LoadContent};
use game::World;
use graphics::renderer::{RenderFrame, Renderer};

//буря-engine
fn main() {
    //let PLEASE = glium::glutin::
    //this is for assets that have been loaded by their threads
    //and then for the content manifest to keep track of them
    let (load_subthread_sender, content_manifest_asset_receiver): (Sender<EContentType>, Receiver<EContentType>) =
        mpsc::channel();

    //this is for the game thread to ask for an asset to be loaded
    //and for the load thread to kick off the loading process
    let (game_thread_request, loading_thread_fulfillment): (Sender<EContentRequestType>, Receiver<EContentRequestType>) =
        mpsc::channel();

    //this is for the render thread to ask the content manifest for an asset
    //and for the content manifest to handle that request
    let (render_thread_asset_request, content_manifest_request_fulfillment): (Sender<EContentLoadRequst>,
                                                                              Receiver<EContentLoadRequst>) = mpsc::channel();

    //this is for the content manifest to send assets that the loading thread has asked for
    //and for the render thread to start using them
    let (content_manifest_fulfillment, render_thread_asset_reciver): (Sender<EContentType>, Receiver<EContentType>) =
        mpsc::channel();

    //this is for the loading thread to send back the content id associated with the asset that the
    //game just asked for
    let (loading_thread_content_id, game_thread_content_id): (Sender<EContentRequestResult>, Receiver<EContentRequestResult>) =
        mpsc::channel();

    //this is for the game thread to use to send over frames it wants rendered
    let (game_thread_render_frame, render_thread_render_frame): (SyncSender<RenderFrame>, Receiver<RenderFrame>) =
        mpsc::sync_channel(3);

    let (game_input_thread, game_thread_gets_input): (Sender<glutin::VirtualKeyCode>, Receiver<glutin::VirtualKeyCode>) =
        mpsc::channel();


    let _ = thread::spawn(move || {
        ContentManifest::thread_loop(
            content_manifest_asset_receiver,
            content_manifest_request_fulfillment,
            content_manifest_fulfillment.clone(),
        )
    });


    //create a content loader
    let load_content = LoadContent::new(
        loading_thread_fulfillment,
        loading_thread_content_id.clone(),
        load_subthread_sender.clone(),
    );

    let _ = thread::spawn(move || { LoadContent::thread_loop(load_content); });

    //create a render loop



    let _ = thread::spawn(move || {
        World::update(
            game_thread_request,
            game_thread_content_id,
            game_thread_render_frame.clone(),
            game_thread_gets_input,
        );
    });

    Renderer::render_thread(
        render_thread_render_frame,
        render_thread_asset_request.clone(),
        render_thread_asset_reciver,
        game_input_thread,
    );
}
