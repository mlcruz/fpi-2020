#[cfg(test)]
mod tests {
    use std::{error::Error, path::Path};

    use image::{DynamicImage, GenericImage, GenericImageView};

    #[test]
    fn read_as_bytes() -> Result<(), Box<dyn Error>> {
        let image_folder_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/images/1/");

        let img = image::open(dbg!(image_folder_path.join("Gramado_22k.jpg")))?;

        let mut new_img = DynamicImage::new_rgb8(1, 1);
        new_img.put_pixel(0, 0, img.get_pixel(0, 0));

        dbg!(new_img.as_bytes());
        dbg!(new_img.get_pixel(0, 0));

        Ok(())
    }
}
