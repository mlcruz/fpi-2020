use std::{collections::HashMap, path::Path, rc::Rc};

use druid::{
    piet::ImageFormat,
    widget::{Flex, Image, Label, WidgetExt},
    AppLauncher, ImageBuf, Widget, WindowDesc,
};
use image::{DynamicImage, GenericImageView};

pub fn main() {
    let image_folder_path = Path::new(&std::env::current_dir().unwrap())
        .to_path_buf()
        .join("src/images/1/");

    let mut images = HashMap::new();

    for image_path in std::fs::read_dir(image_folder_path).unwrap() {
        let image_path = image_path.unwrap();
        let image = image::open(image_path.path()).unwrap();

        images.insert(image_path.file_name().to_str().unwrap().to_owned(), image);
    }

    let data = 100u64;
    let main_window = WindowDesc::new(move || ui_builder(&images));
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder(images: &HashMap<String, DynamicImage>) -> impl Widget<u64> {
    let mut col = Flex::column();

    let mut row = Flex::row();

    let label = Label::new(|data: &u64, _env: &_| format!("{}", data));

    let gramado_22k = images.get("Space_187k.jpg").unwrap().clone();
    let (width, heigth) = get_dimensions(&gramado_22k);

    let img = convert_dynamic(&gramado_22k).fix_width(width as f64);

    let flip_h = convert_dynamic(&gramado_22k.brighten(10));
    row.add_flex_child(img, 1.0);
    row.add_default_spacer();
    row.add_flex_child(flip_h, 1.0);

    col.add_child(label);
    col.add_child(row);

    // let img = Image::new(png_data.clone())
    //     .border(Color::WHITE, 1.0)
    //     .fix_width(100.0)
    //     .center();

    // let otherimage = Image::new(png_data)
    //     .fill_mode(FillStrat::FitWidth)
    //     .border(Color::WHITE, 1.0);
    // col.add_flex_child(otherimage, 1.0);
    col
}

pub fn convert_dynamic(img: &DynamicImage) -> Image {
    let (ax, ay, bx, by) = img.bounds();
    let width = bx - ax;
    let height = by - ay;

    let pixels = img.as_bytes();

    Image::new(ImageBuf::from_raw(
        pixels,
        ImageFormat::Rgb,
        width as usize,
        height as usize,
    ))
}

pub fn get_dimensions(img: &DynamicImage) -> (u32, u32) {
    let (ax, ay, bx, by) = img.bounds();
    let width = bx - ax;
    let height = by - ay;

    (width, height)
}
