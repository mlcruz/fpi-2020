use image::{DynamicImage, GenericImageView, GrayImage, Luma};

use crate::imageops::ImageExt;

pub trait ImageExt2 {
    fn render_grayscale_histogram(&self) -> DynamicImage;
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
