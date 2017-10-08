
use std::sync::mpsc::{Receiver, Sender};

use gfx;
use glutin;
use gfx_window_glutin;
use std::collections::HashMap;

use gfx::{Adapter, CommandQueue, Device, FrameSync,
          Surface, Swapchain, SwapchainExt, WindowExt};
use gfx_device_gl;
use image;

use game::ContentId;
use content::load_content::{EContentType, EContentLoadRequst};
use graphics::box_renderer::BoxRenderData;
use graphics::sphere_renderer::{SphereRenderData, SphereRenderer};

use glutin::{VirtualKeyCode};

use frame_timer::FrameTimer;

use graphics::box_renderer::BoxRenderer;
use cgmath::{self};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


pub struct RenderPackage<'a> {
    pub device: &'a mut gfx_device_gl::Device,
    pub graphics_queue: &'a mut gfx::queue::GraphicsQueue<gfx_device_gl::Backend>,

    pub frame_semaphore: &'a gfx::handle::Semaphore<gfx_device_gl::Resources>,
    pub draw_semaphore: &'a gfx::handle::Semaphore<gfx_device_gl::Resources>,
    pub frame_fence: &'a gfx::handle::Fence<gfx_device_gl::Resources>,
}


impl<'a> RenderPackage<'a> {
    pub fn new(device: &'a mut gfx_device_gl::Device,
                graphics_queue: &'a mut gfx::queue::GraphicsQueue<gfx_device_gl::Backend>,
                frame_semaphore: &'a gfx::handle::Semaphore<gfx_device_gl::Resources>,
                draw_semaphore: &'a gfx::handle::Semaphore<gfx_device_gl::Resources>,
                frame_fence: &'a gfx::handle::Fence<gfx_device_gl::Resources>,
) -> RenderPackage<'a> {

        RenderPackage {
            device,
            graphics_queue,
            frame_semaphore,
            draw_semaphore,
            frame_fence
        }
    }
}

#[derive(Clone)]
pub struct RenderFrame {
    pub frame_index: u64,
    pub boxes: Option<Vec<BoxRenderData>>,
    pub spheres: Option<Vec<SphereRenderData>>,
}

impl RenderFrame {
    pub fn new(frame_index: u64, boxes: Option<Vec<BoxRenderData>>, spheres: Option<Vec<SphereRenderData>>) -> RenderFrame {
        RenderFrame {
            frame_index: frame_index,
            boxes,
            spheres
        }
    }
}

pub struct RenderThread {
    from_game_thread: Receiver<RenderFrame>,
    _to_content_manifest: Sender<EContentLoadRequst>,
    _from_content_manifest: Receiver<EContentType>,
    to_game_thread_with_input: Sender<VirtualKeyCode>,
    _current_frame_index: u64,
    textures: HashMap<ContentId,gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32;4]>>,
    pub use_matrix : [[f32;4];4],
}

impl RenderThread {

    pub fn new(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<VirtualKeyCode>,
    ) -> RenderThread {
        let o = cgmath::ortho(-2000.0f32, 2000.0f32, -2000.0f32, 2000.0f32, 0.0, 10.0);
        RenderThread {
            _current_frame_index: 0,
            from_game_thread: from_game_thread,
            _to_content_manifest: to_content_manifest,
            _from_content_manifest: from_content_manifest,
            to_game_thread_with_input: to_game_thread_with_input,
            textures: HashMap::new(),
            use_matrix: [
                [o.x[0], o.x[1], o.x[2], o.x[3]],
                [o.y[0], o.y[1], o.y[2], o.y[3]], 
                [o.z[0], o.z[1], o.z[2], o.z[3]], 
                [o.w[0], o.w[1], o.w[2], o.w[3]] 
            ]
        }
    }

    pub fn load_texture<D, R>(factory: &mut D, path: &str) -> gfx::handle::ShaderResourceView<R, [f32; 4]> where D: gfx::Device<R>, R: gfx::Resources
    {
        let img = image::open(path).unwrap().to_rgba();
        let (width, height) = img.dimensions();
        let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
        let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&img]).unwrap();
        view
    }

    pub fn query_content_manifest_for_sprite(&mut self, _content_id: ContentId) -> bool {

        return false;        
        /*
        if self.sprites.contains_key(&content_id) {
            true
        } else {
            let _ = self.to_content_manifest.send(EContentLoadRequst::Image(
                content_id,
            ));
            let value = self.from_content_manifest.recv().unwrap();
            match value {
                EContentType::Image(id, dy_image) => {
                    /*
                    let image_dimensions = dy_image.to_rgba().dimensions();
                    let loaded_image = glium::texture::RawImage2d::from_raw_rgba_reversed(dy_image.to_rgba().into_raw(), image_dimensions);
                    let tex = Texture2d::new(&self.display, loaded_image).unwrap();
                    let spr = Sprite::new("Sprite".to_string(), tex, &self.display);
                    self.sprites.insert(id, spr);
                    */

                    true
                }
                EContentType::NotLoaded => false,
            }
        }
    */
    }


    pub fn thread_loop(
        from_game_thread: Receiver<RenderFrame>,
        to_content_manifest: Sender<EContentLoadRequst>,
        from_content_manifest: Receiver<EContentType>,
        to_game_thread_with_input: Sender<VirtualKeyCode>,
    ) {
        let mut rend =
            RenderThread::new(from_game_thread, to_content_manifest, from_content_manifest, to_game_thread_with_input);

        rend.render();
    }


    pub fn render(&mut self) {
        let mut frame_timer = FrameTimer::new();
        let mut events_loop = glutin::EventsLoop::new();

        

        let builder = glutin::WindowBuilder::new()
            .with_title("Square Toy".to_string())
            .with_dimensions(800, 800);

        let gl_builder = glutin::ContextBuilder::new().with_vsync(true);
        let windows = glutin::GlWindow::new(builder, gl_builder, &events_loop).unwrap();
//        let context = glutin::ContextBuilder::new().with_vsync(true);

        //let test_window = ;
        let (mut surface, adapters) = gfx_window_glutin::Window::new(windows).get_surface_and_adapters();
  
        let gfx::Gpu{mut device, mut graphics_queues, ..} = 
            adapters[0].open_with(|family, ty| {
                ((ty.supports_graphics() && surface.supports_queue(&family)) as u32, gfx::QueueType::Graphics)
            });
        
        let mut graphics_queue = graphics_queues.pop().expect("Unable to find a graphics queue");
        let config = gfx::SwapchainConfig::new().with_color::<ColorFormat>();
        let mut swap_chain = surface.build_swapchain(config, &graphics_queue);
        let views : Vec<gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>> = swap_chain.create_color_views(&mut device);

        let mut box_rend = BoxRenderer::new(&mut device, graphics_queue.create_graphics_pool(1));
        let mut sphere_rend = SphereRenderer::new(&mut device, graphics_queue.create_graphics_pool(1));

        let frame_semaphore = device.create_semaphore();
        let draw_semaphore = device.create_semaphore();
        let frame_fence = device.create_fence(false);
        let mut running = true;
        
        let mut frame;
        let mut frame_data;

        while running {
           
            let mut render_package = RenderPackage::new(&mut device, &mut graphics_queue, &frame_semaphore, &draw_semaphore, &frame_fence);
            frame_timer.frame_start();            
            //the first thing we do is grab the current frame

            events_loop.poll_events(|event| {
            
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput{device_id: _id_of_device, input: input_event} => {
                        if input_event.virtual_keycode.is_none() == true {
                            return;
                        }

                        if input_event.virtual_keycode.unwrap() == VirtualKeyCode::Escape {
                            running = false;
                        }

                        let _ = self.to_game_thread_with_input.send(input_event.virtual_keycode.unwrap());
                    },
                    glutin::WindowEvent::Resized(_width, _height) => {
                        // TODO
                    },
                    _ => (),
                }
            }
            });

            frame = swap_chain.acquire_frame(FrameSync::Semaphore(&frame_semaphore));
            let frame_view = &views[frame.id()].clone();



            frame_data = self.from_game_thread.try_recv();


            let frame_data = match frame_data {
                Ok(data) => Some(data),
                Err(_) => {
                    None
                }
            };
            
            if frame_data.is_some() {
                let frame_data = frame_data.unwrap();
                
                if frame_data.boxes.is_some() {
                  //  let fake : Vec<BoxRenderData> = vec![BoxRenderData{pos: Vector2::new(0.0f32, 0.0f32), scale: Vector2::new(1.0f32, 1.0f32), z_rotation: 0.0f32, color: [1.0f32, 1.0f32, 1.0f32]}];
                    box_rend.render_boxes(&frame_data.boxes.unwrap(), &mut render_package, &frame_view, self);
                }

                if frame_data.spheres.is_some() {
                    sphere_rend.render_spheres(&frame_data.spheres.unwrap(), &mut render_package, &frame_view, self);
                }
            }

            
            swap_chain.present(&mut render_package.graphics_queue, &[&draw_semaphore]);
            render_package.device.wait_for_fences(&[&frame_fence], gfx::WaitFor::All, 1_000_000);
            render_package.graphics_queue.cleanup();
            
            frame_timer.frame_end();
        }
    }
}


/*
gfx_defines! {
    vertex VertexColor {
        pos: [f32;2] = "a_Pos",
        color: [f32;3] = "a_Color",
    }

    pipeline pipe_color {
        vbuf: gfx::VertexBuffer<VertexColor> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }

    constant Transform {
        transform:  [[f32;4]; 4] = "u_Transform",
        scale:      [[f32;4]; 4] = "u_Scale",
        rotation_z: [[f32;4]; 4] = "u_Rotation_z",
    }

    pipeline pipe_sin {
        vbuf: gfx::VertexBuffer<VertexColor> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
*/