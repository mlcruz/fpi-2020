use druid::{piet::ImageFormat, widget::Image, Data, ImageBuf};
use image::{
    DynamicImage, EncodableLayout, GenericImage, GenericImageView, GrayImage, Luma, RgbImage, Rgba,
};

use crate::imageops2::Kernel;

pub trait ImageExt {
    fn flip_v(&self) -> DynamicImage;
    fn flip_h(&self) -> DynamicImage;
    fn to_grayscale(&self) -> DynamicImage;
    fn to_grayscale_rgb(&self) -> DynamicImage;
    fn get_dimensions(&self) -> (u32, u32);
    fn quantize_grayscale(&self, qty: u8) -> DynamicImage;
}

impl ImageExt for DynamicImage {
    fn flip_v(&self) -> DynamicImage {
        let mut raw = self.clone().into_bytes();

        let (w, h) = self.dimensions();
        let stride = (w * 3) as usize;
        let len = (stride * h as usize / 2) as usize;
        let slice_h = h as usize / 2;

        let (first, second) = raw.split_at_mut(len);

        for idx in 0..(h as usize / 2) {
            let mut row_upper = &mut first[idx * stride..(idx + 1) * stride];
            let row_lower = &mut second[(slice_h - idx - 1) * stride..(slice_h - idx) * stride];

            row_lower.swap_with_slice(&mut row_upper);
        }

        DynamicImage::ImageRgb8(RgbImage::from_raw(w, h, raw).unwrap())
    }

    fn flip_h(&self) -> DynamicImage {
        let (w, h) = self.dimensions();
        let mut new_img = DynamicImage::new_rgb8(w, h);

        for (x, y, pixel) in self.pixels() {
            new_img.put_pixel((w - 1 - x) as u32, y, pixel);
        }

        new_img
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

    fn to_grayscale_rgb(&self) -> DynamicImage {
        let (width, height) = self.get_dimensions();

        let mut new_img = DynamicImage::new_rgb8(width, height);

        for (x, y, pixel) in self.pixels() {
            let new_l = ((0.299 * pixel[0] as f64) as u64
                + (0.587 * pixel[1] as f64) as u64
                + (0.114 * pixel[2] as f64) as u64) as u8;

            new_img.put_pixel(x, y, Rgba::from([new_l, new_l, new_l, 1]));
        }

        new_img
    }

    fn get_dimensions(&self) -> (u32, u32) {
        let (ax, ay, bx, by) = self.bounds();
        let width = bx - ax;
        let height = by - ay;

        (width, height)
    }

    fn quantize_grayscale(&self, qty: u8) -> DynamicImage {
        let (width, height) = self.get_dimensions();
        let mut grayscale = self.to_grayscale().as_luma8().unwrap().clone().into_raw();
        let len = grayscale.len();
        let (min, max) = {
            let mut tmp_max = 0;
            let mut tmp_min = 255;
            for l in &grayscale {
                if l < &tmp_min {
                    tmp_min = *l;
                };

                if l > &tmp_max {
                    tmp_max = *l;
                };
            }

            (tmp_min, tmp_max)
        };

        let interval_size = max - min + 1;

        if qty > interval_size {
            return self.to_grayscale_rgb();
        }

        let bin_size = interval_size / qty;

        // qty 64
        // min 64
        // max 196
        // interval = 128
        // bin size = 2
        // (64 - min) / 2 -> 0
        // (65 - min ) / 2 -> 0
        // (66 - min ) / 2 -> 1
        // ...
        // (196 - min) / 2 -> 64
        for idx in 0..len {
            // 255
            let l = grayscale[idx];
            let bin_idx = (l - min) / bin_size;

            let bin_value = min + (bin_idx * bin_size);
            grayscale[idx] = bin_value;
        }

        DynamicImage::ImageLuma8(GrayImage::from_raw(width, height, grayscale).unwrap())
    }
}

#[derive(Debug, Data, Clone, Copy, PartialEq)]
pub enum Operation {
    None,
    FlipH,
    FlipV,
    Save,
    Grayscale,
    Quantize,
    AdjustBrightness,
    AdjustContrast,
    Negative,
    ZoomOut,
    ZoomIn,
    Convolution(Kernel),
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
