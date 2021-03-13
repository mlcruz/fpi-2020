use core::panic;
use std::{
    cmp::{max, min},
    vec,
};

use image::{
    DynamicImage, EncodableLayout, GenericImage, GenericImageView, GrayImage, Luma, Pixel,
    RgbImage, Rgba,
};

use crate::imageops::ImageExt;

pub trait ImageExt2 {
    fn render_grayscale_histogram(&self) -> DynamicImage;
    fn adjust_brightness(&self, val: u8) -> DynamicImage;
    fn adjust_contrast_2(&self, val: u8) -> DynamicImage;
    fn negative(&self) -> DynamicImage;
    fn zoom_out(&self, x: u8, y: u8) -> DynamicImage;
}

impl ImageExt2 for DynamicImage {
    fn render_grayscale_histogram(&self) -> DynamicImage {
        let grayscale = self.to_grayscale();
        let mut histogram: [u32; 256] = [0; 256];

        let (w, h) = self.dimensions();
        let size = w * h;

        for l in grayscale.as_bytes() {
            histogram[*l as usize] += 1;
        }

        // maximum value is going to be our full column
        let max_val = histogram.iter().max().unwrap();
        let pixel_value = (*max_val as f64) / 255.0;

        let mut result_image = GrayImage::from_raw(256, 256, [255; 256 * 256].to_vec()).unwrap();

        for (col, count) in histogram.iter().enumerate() {
            let column_height = (*count as f64 / pixel_value).ceil() as u8;
            dbg!(column_height);

            for row in 0..column_height {
                result_image.put_pixel(col as u32, (255 - row) as u32, Luma::from([0]));
            }
        }

        debug_assert!((|| {
            let column_sum: usize = histogram
                .iter()
                .fold(0, |acc: usize, cur| (acc + (*cur as usize)) as usize);

            column_sum == size as usize
        })());

        image::DynamicImage::ImageLuma8(result_image)
    }

    fn adjust_brightness(&self, val: u8) -> DynamicImage {
        let mut new_img = self.clone();

        let adjust_pixel = |p: u8| {
            let result = min(255, max(0, p as i32 + val as i32));
            debug_assert!(result <= 255 && result >= 0);
            result as u8
        };

        for (x, y, pixel) in self.pixels() {
            let (r, g, b, a) = pixel.channels4();
            new_img.put_pixel(
                x,
                y,
                Rgba::from([adjust_pixel(r), adjust_pixel(g), adjust_pixel(b), a]),
            );
        }

        new_img
    }

    fn adjust_contrast_2(&self, val: u8) -> DynamicImage {
        let mut new_img = self.clone();

        let adjust_pixel = |p: u8| {
            let result = min(255, max(0, p as i32 * val as i32));
            debug_assert!(result <= 255 && result >= 0);
            result as u8
        };

        for (x, y, pixel) in self.pixels() {
            let (r, g, b, a) = pixel.channels4();
            new_img.put_pixel(
                x,
                y,
                Rgba::from([adjust_pixel(r), adjust_pixel(g), adjust_pixel(b), a]),
            );
        }

        new_img
    }

    fn negative(&self) -> DynamicImage {
        let mut new_img = self.clone();

        let adjust_pixel = |p: u8| {
            let result = min(255, max(0, 255 - p));
            result
        };

        for (x, y, pixel) in self.pixels() {
            let (r, g, b, a) = pixel.channels4();
            new_img.put_pixel(
                x,
                y,
                Rgba::from([adjust_pixel(r), adjust_pixel(g), adjust_pixel(b), a]),
            );
        }

        new_img
    }

    fn zoom_out(&self, scaling_w: u8, scaling_h: u8) -> DynamicImage {
        let (w, h) = self.dimensions();
        let src_bytes = self.to_rgb8();

        let mut buffer: Vec<u8> = vec![];
        let new_w = w / scaling_w as u32;
        let new_h = h / scaling_h as u32;

        let rows = src_bytes
            .as_bytes()
            .chunks_exact((w * scaling_h as u32 * 3) as usize);

        let get_block = |row: &[u8], offset: usize| {
            // Get a block of NxM pixels from some source as a flat array
            let mut buf = Vec::with_capacity((scaling_w * scaling_h * 3) as usize);

            for i in 0..scaling_h as usize {
                let col_idx = (i * 3 * w as usize) + offset;

                if (col_idx > 2870) {
                    let foo = true;
                }

                for j in 0..(scaling_w as usize * 3) {
                    let value = row[col_idx + j];
                    buf.push(value)
                }
            }

            buf
        };

        for row in rows {
            for offset in (0..(3 * w) as usize).step_by((scaling_w * 3) as usize) {
                let block = get_block(row, offset as usize);
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for pixel in block.chunks_exact(3) {
                    r += pixel[0] as u32;
                    g += pixel[1] as u32;
                    b += pixel[2] as u32;
                }

                let block_size = scaling_h as u32 * scaling_w as u32;
                buffer.push((r / block_size) as u8);
                buffer.push((g / block_size) as u8);
                buffer.push((b / block_size) as u8);
            }
        }

        let image = RgbImage::from_raw(new_w, new_h, buffer).unwrap();

        DynamicImage::ImageRgb8(image)
    }
}

#[cfg(test)]
mod tests {
    use crate::imageops2::ImageExt2;

    use std::{error::Error, path::Path};

    #[test]
    fn test_render_histogram() -> Result<(), Box<dyn Error>> {
        let image_folder_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/images/1/");

        let img = image::open(dbg!(image_folder_path.join("Gramado_22k.jpg")))?;

        img.render_grayscale_histogram();

        Ok(())
    }
}
