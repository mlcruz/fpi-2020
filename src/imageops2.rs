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

pub type Kernel = [f32; 9];

pub trait ImageExt2 {
    fn render_grayscale_histogram(&self) -> DynamicImage;
    fn adjust_brightness(&self, val: u8) -> DynamicImage;
    fn adjust_contrast_2(&self, val: u8) -> DynamicImage;
    fn negative(&self) -> DynamicImage;
    fn zoom_out(&self, x: u8, y: u8) -> DynamicImage;
    fn zoom_in(&self) -> DynamicImage;
    fn convolution(&self, kernel: Kernel) -> DynamicImage;
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

                for j in 0..(scaling_w as usize * 3) {
                    let value = row[col_idx + j];
                    buf.push(value);
                }
            }
            debug_assert!(buf.len() == (scaling_w * scaling_h * 3) as usize);
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

    fn zoom_in(&self) -> DynamicImage {
        let (w, h) = self.dimensions();
        let mut new_img = image::DynamicImage::new_rgb8((w * 2) - 1, (h * 2) - 1);
        let empty = new_img.clone();

        let interpolate = |x: Rgba<u8>, y: Rgba<u8>| {
            let x = x.channels();
            let y = y.channels();

            let new_r = ((x[0] as u32 + y[0] as u32) / 2) as u8;
            let new_g = ((x[1] as u32 + y[1] as u32) / 2) as u8;
            let new_b = ((x[2] as u32 + y[2] as u32) / 2) as u8;

            Rgba::from([new_r, new_g, new_b, 1])
        };

        for (x, y, pixel) in self.pixels() {
            new_img.put_pixel(x * 2, y * 2, pixel);
        }

        for (x, y, _) in empty.pixels() {
            // Preenchemos as colunas impares e linhas pares
            if ((x % 2) == 1) && ((y % 2) == 0) {
                let after = new_img.get_pixel(x + 1, y);
                let before = new_img.get_pixel(x - 1, y);
                new_img.put_pixel(x, y, interpolate(before, after));
            }

            // Preenchemos as colunas pares e linhas impares
            if ((x % 2) == 0) && ((y % 2) == 1) {
                let after = new_img.get_pixel(x, y + 1);
                let before = new_img.get_pixel(x, y - 1);
                new_img.put_pixel(x, y, interpolate(before, after));
            }
        }

        for (x, y, _) in empty.pixels() {
            // Preenchemos as colunas impares e linhas impares
            if ((x % 2) == 1) && ((y % 2) == 1) {
                let after = new_img.get_pixel(x, y + 1);
                let before = new_img.get_pixel(x, y - 1);
                let y_interpolated = interpolate(before, after);

                let after = new_img.get_pixel(x + 1, y);
                let before = new_img.get_pixel(x - 1, y);
                let x_interpolated = interpolate(after, before);

                let interpolated = interpolate(x_interpolated, y_interpolated);
                new_img.put_pixel(x, y, interpolated);
            }
        }
        new_img
    }

    fn convolution(&self, kernel: Kernel) -> DynamicImage {
        let (w, h) = self.dimensions();
        // relative kernel weight position to each pixel
        let k_pos = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut new_img = self.to_grayscale_rgb();
        let clamp = |rgb: (f32, f32, f32)| {
            {
                (
                    min(255, max(rgb.0 as i32, 0)) as u8,
                    min(255, max(rgb.1 as i32, 0)) as u8,
                    min(255, max(rgb.2 as i32, 0)) as u8,
                )
            }
        };

        let mut apply_kernel = |x, y| {
            let mut sum = (0.0, 0.0, 0.0);

            for (idx, w) in kernel.iter().enumerate() {
                let x = (x as i32 + k_pos[idx].0) as u32;
                let y = (y as i32 + k_pos[idx].1) as u32;

                let pixel = self.get_pixel(x, y);

                sum.0 += (pixel[0] as f32) * w;
                sum.1 += (pixel[1] as f32) * w;
                sum.2 += (pixel[2] as f32) * w;
            }
            let sum = clamp(sum);

            new_img.put_pixel(x, y, Rgba::from([sum.0 as u8, sum.1 as u8, sum.2 as u8, 1]));
        };

        for (x, y, _) in self.pixels() {
            if x == 0 || y == 0 || x == (w - 1) || y == (h - 1) {
                continue;
            }

            apply_kernel(x, y);
        }

        new_img
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
