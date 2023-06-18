use image::open;

pub struct ImageInformation {
    pub height: u32,
    pub width: u32,
}

pub struct Img;
impl Img {
    pub fn get_image_meta(path: String) -> ImageInformation {
        let target_image = open(path).unwrap();
        let final_struct = ImageInformation {height: target_image.height(), width: target_image.width()};
        final_struct
    }
}
