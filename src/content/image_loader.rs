use graphics::sprite::Sprite;
use game::ContentId;

pub struct ImageLoader {
    sprites: HashMap<ContentId, Sprite>,
    name_to_content_id: HashMap<String, ContentId>
}

impl ImageLoader {
    pub fn GetSprite(&mut self, image_location: String) -> Option<Sprite> {
        
    }

    pub fn GetSpriteByContentId(&mut self, content_id: ContentId) -> Option<Sprite> {

    }
}

