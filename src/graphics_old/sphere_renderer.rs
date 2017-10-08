use cgmath::Vector2;
use gfx;
use gfx_device_gl;
use gfx_device_gl::{Resources};
use gfx::{GraphicsPoolExt};
use gfx::traits::DeviceExt;

use graphics::render_thread::{RenderPackage, RenderThread};
type ColorFormat = gfx::format::Rgba8;

gfx_defines!{
    vertex SphereVertex {
        pos: [f32;2] = "a_Pos",
        uv: [f32; 2] = "uv",
        color: [f32;3] = "color",
    }

    constant Transform {
        proj: [[f32;4];4] = "u_prop",
    }

    pipeline SpherePipeLine {
        vbuf: gfx::VertexBuffer<SphereVertex> = (),
        proj_uni: gfx::ConstantBuffer<Transform> = "Proj",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

#[derive(Clone)]
pub struct SphereRenderData {
    pub pos: Vector2<f32>,
    pub scale: f32,
    pub color: [f32; 3],
}

pub struct SphereRenderer {
    pso: gfx::PipelineState<Resources, SpherePipeLine::Meta>,
    graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>,
}

impl SphereRenderer {
    
    pub fn new(device: &mut gfx_device_gl::Device, graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>) -> SphereRenderer {

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
            pso,
            graphics_pool
        }
        
    }

    pub fn render_spheres(&mut self, boxes_to_render: &Vec<SphereRenderData>, render_package: &mut RenderPackage, view: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>, rt: &mut RenderThread) {
        
        let mut vertex_info = vec![];
        let mut index_info : Vec<u16> = vec![];


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
        let t = Transform {
            proj: rt.use_matrix,
        };
        let constant_buffer  = render_package.device.create_constant_buffer(1);

        let box_data = SpherePipeLine::Data {
            vbuf: vertex_buffer.clone(),
            proj_uni: constant_buffer,
            out: view.clone(),
        };

        {
            let mut sphere_encoder = self.graphics_pool.acquire_graphics_encoder();
            let _ = sphere_encoder.update_buffer(&box_data.proj_uni, &[t], 0);
            sphere_encoder.draw(&index_buffer, &self.pso, &box_data);
            let _ = sphere_encoder.synced_flush(render_package.graphics_queue, &[&render_package.frame_semaphore.clone()], &[&render_package.draw_semaphore.clone()], Some(&render_package.frame_fence.clone())).expect("could not flush encoder");
        }

        self.graphics_pool.reset();
    }
}