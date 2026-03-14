use slint::{Image, SharedPixelBuffer};

pub fn image_to_slint(img: image::DynamicImage) -> Image {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let buffer = SharedPixelBuffer::clone_from_slice(rgba.as_raw(), width, height);

    Image::from_rgba8(buffer)
}
