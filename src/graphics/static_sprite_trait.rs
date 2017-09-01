use graphics::sprite_renderer::SpriteRenderData;

pub trait StaticSprite {
    fn generate_sprite_render_data(&self) -> SpriteRenderData;
}
