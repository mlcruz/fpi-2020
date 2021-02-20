#[cfg(test)]
mod tests {
    use std::{error::Error, path::Path};

    use image::GenericImageView;

    #[test]
    fn assert_saving_eq() -> Result<(), Box<dyn Error>> {
        let image_folder_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/images/1/");

        let verification_folder_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/verification_images/");

        let img = image::open(dbg!(verification_folder_path.join("Gramado_22k_2.jpg")))?;
        img.save(verification_folder_path.join("Gramado_22k_3.jpg"))
            .unwrap();

        let saved_img = image::open(verification_folder_path.join("Gramado_22k_3.jpg")).unwrap();

        let pxels = img.pixels().into_iter().collect::<Vec<_>>();

        let pxels_saved = saved_img.pixels().into_iter().collect::<Vec<_>>();

        for idx in 2..pxels.len() {
            assert_eq!(pxels[0], pxels_saved[1]);
        }

        assert_eq!(pxels, pxels_saved);

        Ok(())
    }
}
