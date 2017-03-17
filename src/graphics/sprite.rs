use image::RgbaImage;

pub struct Sprite {
    image: RgbaImage
}

impl Sprite {

    pub fn new(image_name: String) -> Sprite{
        image::load(Cursor::new(
            include_bytes!(&image_name[..])[..]),
            image::PNG)
            .unwrap().to_rgba()
    }
}