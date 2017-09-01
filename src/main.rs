//#![feature(test)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_app;
extern crate glutin;
extern crate glium;
extern crate image;
extern crate cgmath;
//extern crate test;
extern crate threadpool;


#[macro_use]
mod macros;

pub mod game;
pub mod graphics;
pub mod math;
pub mod physics;
pub mod content;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use content::load_content::{EContentType, EContentRequestType, EContentRequestResult,
                            EContentLoadRequst};
use content::{ContentManifest, LoadContent};
use graphics::RenderThread;
use graphics::render_thread::RenderFrame;
use game::World;

fn main() {

    //let PLEASE = glium::glutin::
    //this is for assets that have been loaded by their threads
    //and then for the content manifest to keep track of them
    let (load_subthread_sender, content_manifest_asset_receiver): (Sender<EContentType>,
                                                                   Receiver<EContentType>) =
        mpsc::channel();

    //this is for the game thread to ask for an asset to be loaded
    //and for the load thread to kick off the loading process
    let (game_thread_request, loading_thread_fulfillment): (Sender<EContentRequestType>,
                                                            Receiver<EContentRequestType>) =
        mpsc::channel();

    //this is for the render thread to ask the content manifest for an asset
    //and for the content manifest to handle that request
    let (render_thread_asset_request, content_manifest_request_fulfillment)
            : (Sender<EContentLoadRequst>, Receiver<EContentLoadRequst>)
            = mpsc::channel();

    //this is for the content manifest to send assets that the loading thread has asked for
    //and for the render thread to start using them
    let (content_manifest_fulfillment, render_thread_asset_reciver): (Sender<EContentType>,
                                                                      Receiver<EContentType>) =
        mpsc::channel();

    //this is for the loading thread to send back the content id associated with the asset that the
    //game just asked for
    let (loading_thread_content_id, game_thread_content_id): (Sender<EContentRequestResult>,
                                                              Receiver<EContentRequestResult>) =
        mpsc::channel();

    //this is for the game thread to use to send over frames it wants rendered
    let (game_thread_render_frame, render_thread_render_frame): (Sender<RenderFrame>,
                                                                 Receiver<RenderFrame>) =
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
        );
    });

    RenderThread::thread_loop(
        render_thread_render_frame,
        render_thread_asset_request.clone(),
        render_thread_asset_reciver,
    );
}
