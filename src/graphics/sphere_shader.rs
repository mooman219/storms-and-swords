use cgmath::Vector2;
use gfx;
use gfx_device_gl;
use gfx_device_gl::{Resources};
use gfx::{Device, CommandQueue,FrameSync, GraphicsPoolExt,
          Surface, Swapchain, SwapchainExt, WindowExt};
use gfx::traits::DeviceExt;

use graphics::render_thread::RenderPackage;
type ColorFormat = gfx::format::Rgba8;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
gfx_defines!{
    vertex SphereVertex {
        pos: [f32;2] = "a_Pos",
        uv: [f32; 2] = "uv",
        color: [f32;3] = "color",
    }

    pipeline SpherePipeLine {
        vbuf: gfx::VertexBuffer<SphereVertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct SphereRenderData {
    pub pos: Vector2<f32>,
    pub scale: f32,
    pub color: [f32; 3],
}

pub struct SphereRenderer {
    pso: gfx::PipelineState<Resources, SpherePipeLine::Meta>,
}

impl SphereRenderer {
    
    pub fn new(device: &mut gfx_device_gl::Device) -> SphereRenderer {

        let pso = device.create_pipeline_simple(
            include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/sphere_shader.vs"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/sphere_shader.fs"
                )),
                SpherePipeLine::new(),
        ).unwrap();

        SphereRenderer {
            pso
        }
        
    }

    pub fn render_boxes(&mut self, boxes_to_render: Vec<SphereRenderData>, render_package: &mut RenderPackage, view: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,) {
        
        let mut vertex_info = vec![];
        let mut index_info : Vec<u16> = vec![];

        let mut graphics_pool = render_package.graphics_queue.create_graphics_pool(1);
        let mut sphere_encoder = graphics_pool.acquire_graphics_encoder();

        for box_to_render in boxes_to_render.iter() {
            vertex_info.extend(&[
                SphereVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale)], uv:[0.0f32, 0.0f32], color: box_to_render.color},//top left
                SphereVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale)], uv:[1.0f32, 0.0f32], color: box_to_render.color},//top right
                SphereVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale)], uv:[0.0f32, 1.0f32], color: box_to_render.color},//bottom left
                SphereVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale)], uv:[1.0f32, 1.0f32], color: box_to_render.color}//bottom right
                ]
            );
        }

        for i in 0..boxes_to_render.len() {
            let i = i as u16;
            index_info.extend(&[0 + (i * 4), 1 + (i * 4), 2 + (i * 4),//top left triangle
                                2 + (i * 4), 1 + (i * 4), 3 + (i * 4)]);//bottom right triangle
        }

        let (vertex_buffer, index_buffer) = render_package.device.create_vertex_buffer_with_slice(&vertex_info, &*index_info);
        
        let box_data = SpherePipeLine::Data {
            vbuf: vertex_buffer.clone(),
            out: view.clone(),
        };

        sphere_encoder.clear(&box_data.out, BLACK);
        sphere_encoder.draw(&index_buffer, &self.pso, &box_data);
        let _ = sphere_encoder.synced_flush(render_package.graphics_queue, &[&render_package.frame_semaphore], &[&render_package.draw_semaphore], Some(&render_package.frame_fence)).expect("could not flush encoder");
    }
}