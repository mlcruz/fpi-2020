use druid::{piet::ImageFormat, widget::Image, Data, ImageBuf};
use image::{
    DynamicImage, EncodableLayout, GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma,
    RgbImage,
};

pub trait ImageExt {
    fn flip_v(&self) -> DynamicImage;
    fn flip_h(&self) -> DynamicImage;
    fn to_grayscale(&self) -> DynamicImage;
    fn get_dimensions(&self) -> (u32, u32);
}

impl ImageExt for DynamicImage {
    fn flip_v(&self) -> DynamicImage {
        self.flipv()
    }

    fn flip_h(&self) -> DynamicImage {
        self.fliph()
    }

    fn to_grayscale(&self) -> DynamicImage {
        let (width, height) = self.get_dimensions();

        let mut new_img: GrayImage = GrayImage::new(width, height);

        for (x, y, pixel) in self.pixels() {
            let new_l = (0.299 * pixel[0] as f64) as u64
                + (0.587 * pixel[1] as f64) as u64
                + (0.114 * pixel[2] as f64) as u64;

            new_img.put_pixel(x, y, Luma([new_l as u8]));
        }

        DynamicImage::ImageLuma8(new_img)
    }

    fn get_dimensions(&self) -> (u32, u32) {
        let (ax, ay, bx, by) = self.bounds();
        let width = bx - ax;
        let height = by - ay;

        (width, height)
    }
}

#[derive(Debug, Data, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    FlipH,
    FlipV,
    Save,
    Grayscale,
}

pub trait ToDruidImage {
    fn to_druid_image(&self) -> Image;
}

impl ToDruidImage for DynamicImage {
    fn to_druid_image(&self) -> Image {
        let (ax, ay, bx, by) = self.bounds();
        let width = bx - ax;
        let height = by - ay;

        let pixels = self.to_rgb8().clone();

        Image::new(ImageBuf::from_raw(
            pixels.as_bytes(),
            ImageFormat::Rgb,
            width as usize,
            height as usize,
        ))
    }
}
