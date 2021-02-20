use std::error::Error;

use image::{
    io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, ImageOutputFormat,
};
fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open("/home/debian/git/fpi/src/images/1/Gramado_22k.jpg")?;

    let (ax, ay, bx, by) = img.bounds();
    let width = bx - ax;
    let height = by - ay;
    println!("width: {} height: {}", width, height);

    Ok(())
}
