use std::{collections::HashMap, path::Path};

use druid::{
    widget::{Button, Flex},
    Color,
};

use druid::{
    piet::ImageFormat,
    widget::{Image, SizedBox},
    BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, Lens, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId,
};
use image::{DynamicImage, GenericImageView};
#[derive(Debug, Data, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    FlipH,
    FlipV,
}

pub trait ToDruidImage {
    fn to_druid_image(&self) -> Image;
}

impl ToDruidImage for DynamicImage {
    fn to_druid_image(&self) -> Image {
        let (ax, ay, bx, by) = self.bounds();
        let width = bx - ax;
        let height = by - ay;

        let pixels = self.as_bytes();

        Image::new(ImageBuf::from_raw(
            pixels,
            ImageFormat::Rgb,
            width as usize,
            height as usize,
        ))
    }
}

#[derive(Debug, Data, Clone, Lens)]
pub struct AppState {
    pub selected_image: Option<String>,
    pub selected_operation: Operation,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_image: None,
            selected_operation: Operation::FlipH,
        }
    }
}

pub struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl Rebuilder {
    pub fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(&data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_image_list() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let image_folder_path = Path::new(&std::env::current_dir().unwrap())
        .to_path_buf()
        .join("src/images/1/");

    for image_path in std::fs::read_dir(image_folder_path).unwrap() {
        let image_path = image_path.unwrap();

        let path_str = image_path.file_name().to_str().unwrap().to_owned();

        let mut row = Flex::row();
        let label = path_str.clone();
        let btn = Button::new(label.clone())
            .on_click(move |_ctx, data: &mut AppState, _env| {
                data.selected_image = label.clone().into()
            })
            .fix_width(250.0);

        row.add_flex_child(btn, 1.0);
        col.add_flex_child(row, 1.0);
    }

    col
}

pub fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    let mut col = Flex::column();
    let mut row = Flex::row();

    let image_folder_path = Path::new(&std::env::current_dir().unwrap())
        .to_path_buf()
        .join("src/images/1/");

    if let Some(image_path) = state.selected_image.clone() {
        let image = image::open(image_folder_path.join(image_path)).unwrap();
        let img = image.to_druid_image();
        let sized = SizedBox::new(img).border(Color::grey(0.6), 2.0).center();

        row.add_flex_child(sized, 1.0);
    };

    let image_list = build_image_list();
    col.add_flex_child(image_list, 1.0);
    col.add_flex_child(row, 1.0);
    col.boxed()
}

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
