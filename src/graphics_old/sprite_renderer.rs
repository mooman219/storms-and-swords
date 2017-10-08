/*
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
    vertex SpriteVertex {
        pos: [f32;2] = "a_Pos",
        color: [f32;3] = "color",
        rotation: f32 = "rotation",
        uv: [f32;2] = "uv",
    }


    pipeline SpritePipeLine {
        vbuf: gfx::VertexBuffer<SpriteVertex> = (),
        sprite: gfx::TextureSampler<[f32; 4]> = "u_tex",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

#[derive(Clone)]
pub struct SpriteRenderData {
    pub pos: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub z_rotation: f32,
    pub color: [f32; 3],
}

pub struct SpriteRenderer {
    pso: gfx::PipelineState<Resources, SpritePipeLine::Meta>,
    graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>,
}

impl SpriteRenderer {
    
    pub fn new(device: &mut gfx_device_gl::Device, graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>) -> SpriteRenderer {

        let pso = device.create_pipeline_simple(
            include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/box_shader.vs"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/box_shader.fs"
                )),
                SpritePipeLine::new(),
        ).unwrap();

        SpriteRenderer {
            pso,
            graphics_pool
        }
        
    }

    pub fn render_sprites(&mut self, sprites_to_render: &Vec<SpriteRenderData>, render_package: &mut RenderPackage, view: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,) {
        
        let mut vertex_info = vec![];
        let mut index_info : Vec<u16> = vec![];

      //  let mut graphics_pool = render_package.graphics_queue.create_graphics_pool(1);

        for box_to_render in sprites_to_render.iter() {
            vertex_info.extend(&[
                SpriteVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation, uv:[0.0f32, 0.0f32]},//top left
                SpriteVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation, uv:[1.0f32, 0.0f32]},//top right
                SpriteVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation, uv:[0.0f32, 1.0f32]},//bottom left
                SpriteVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation, uv:[1.0f32, 1.0f32]}//bottom right
                ]
            );
        }

        for i in 0..sprites_to_render.len() {
            let i = i as u16;
            index_info.extend(&[0 + (i * 4), 1 + (i * 4), 2 + (i * 4),//top left triangle
                                2 + (i * 4), 1 + (i * 4), 3 + (i * 4)]);//bottom right triangle
        }

        let (vertex_buffer, index_buffer) = render_package.device.create_vertex_buffer_with_slice(&vertex_info, &*index_info);

        let text_sampler = render_package.device.create_sampler_linear();
        
        let sprite_data = SpritePipeLine::Data {
            vbuf: vertex_buffer.clone(),
            sprite: (_, text_sampler),
            out: view.clone(),
        };

        {
            let mut box_encoder = self.graphics_pool.acquire_graphics_encoder();
            box_encoder.clear(&sprite_data.out, BLACK);
            box_encoder.draw(&index_buffer, &self.pso, &sprite_data);
            let _ = box_encoder.synced_flush(render_package.graphics_queue, &[&render_package.frame_semaphore], &[&render_package.draw_semaphore], Some(&render_package.frame_fence)).expect("could not flush encoder");
        }
        self.graphics_pool.reset();
    }
}
*/