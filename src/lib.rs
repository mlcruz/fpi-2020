use std::path::Path;

use druid::{
    widget::{Button, Flex, Label, Slider},
    Color, Insets,
};
use imageops::{ImageExt, Operation};
use imageops2::ImageExt2;

use crate::imageops::*;
use druid::{
    widget::SizedBox, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId,
};
use image::DynamicImage;

pub mod imageops;
pub mod imageops2;

#[derive(Debug, Data, Clone, Lens)]
pub struct AppState {
    pub selected_image: Option<String>,
    pub selected_operation: Operation,
    pub last_operation: Operation,
    pub param1: f64,
    pub param2: f64,
    pub param3: f64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_image: None,
            selected_operation: Operation::FlipH,
            param1: 64.0,
            param2: 1.0,
            param3: 1.0,
            last_operation: Operation::FlipH,
        }
    }
}

pub struct UiBuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl UiBuilder {
    pub fn new() -> UiBuilder {
        UiBuilder {
            inner: Flex::column().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_app_ui(&data);
    }
}

impl Widget<AppState> for UiBuilder {
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

fn build_operation_list() -> impl Widget<AppState> {
    let mut col = Flex::column();
    let mut row = Flex::row();

    let build_op_btn = |text, op| {
        Button::new(text).on_click(move |_ctx, data: &mut AppState, _env| {
            if data.selected_operation != Operation::Save {
                data.last_operation = data.selected_operation;
            } else {
                data.selected_operation = data.last_operation;
            }

            data.selected_operation = op;
        })
    };

    row.add_flex_child(build_op_btn("Negativo", Operation::Negative), 1.0);
    row.add_flex_child(build_op_btn("Contraste", Operation::AdjustContrast), 1.0);
    row.add_flex_child(build_op_btn("Grayscale", Operation::Grayscale), 1.0);
    row.add_flex_child(build_op_btn("Brilho", Operation::AdjustBrightness), 1.0);
    row.add_flex_child(build_op_btn("Quantizar", Operation::Quantize), 1.0);
    row.add_flex_child(build_op_btn("ZoomOut", Operation::ZoomOut), 1.0);

    let mut param_row_1 = Flex::row();
    let param_slider = Flex::column()
        .with_flex_child(
            Slider::new().with_range(1.0, 255.0).fix_size(768.0, 50.0),
            1.0,
        )
        .lens(AppState::param1);

    param_row_1.add_flex_child(Flex::column().with_flex_child(param_slider, 1.0), 1.0);
    param_row_1.add_flex_child(
        Flex::column().with_flex_child(
            Label::new(|data: &AppState, _: &_| format!("{}", data.param1 as usize)),
            1.0,
        ),
        1.0,
    );

    let mut param_row_1 = Flex::row();
    let param_slider = Flex::column()
        .with_flex_child(
            Slider::new().with_range(1.0, 255.0).fix_size(768.0, 50.0),
            1.0,
        )
        .lens(AppState::param1);

    param_row_1.add_flex_child(Flex::column().with_flex_child(param_slider, 1.0), 1.0);
    param_row_1.add_flex_child(
        Flex::column().with_flex_child(
            Label::new(|data: &AppState, _: &_| format!("{}", data.param1 as usize)),
            1.0,
        ),
        1.0,
    );

    let mut param_row_2 = Flex::row();
    let param_slider = Flex::column()
        .with_flex_child(
            Slider::new().with_range(1.0, 5.0).fix_size(512.0, 50.0),
            1.0,
        )
        .lens(AppState::param2);

    param_row_2.add_flex_child(Flex::column().with_flex_child(param_slider, 1.0), 1.0);
    param_row_2.add_flex_child(
        Flex::column().with_flex_child(
            Label::new(|data: &AppState, _: &_| format!("{:.0}", data.param2)),
            1.0,
        ),
        1.0,
    );

    let mut param_row_3 = Flex::row();
    let param_slider = Flex::column()
        .with_flex_child(
            Slider::new().with_range(1.0, 5.0).fix_size(512.0, 50.0),
            1.0,
        )
        .lens(AppState::param3);

    param_row_3.add_flex_child(Flex::column().with_flex_child(param_slider, 1.0), 1.0);
    param_row_3.add_flex_child(
        Flex::column().with_flex_child(
            Label::new(|data: &AppState, _: &_| format!("{:.0}", data.param3)),
            1.0,
        ),
        1.0,
    );

    col.add_flex_child(row, 2.0);
    col.add_default_spacer();
    col.add_flex_child(param_row_1, 1.0);
    col.add_default_spacer();
    col.add_flex_child(param_row_2, 1.0);
    col.add_default_spacer();
    col.add_flex_child(param_row_3, 1.0);

    col
}

fn build_image_list() -> impl Widget<AppState> {
    let image_folder_path = Path::new(&std::env::current_dir().unwrap())
        .to_path_buf()
        .join("src/images/1/");

    let mut row = Flex::row();
    for image_path in std::fs::read_dir(image_folder_path).unwrap() {
        let image_path = image_path.unwrap();

        let path_str = image_path.file_name().to_str().unwrap().to_owned();

        let label = path_str.clone();
        let btn = Button::new(label.clone())
            .on_click(move |_ctx, data: &mut AppState, _env| {
                data.selected_image = label.clone().into()
            })
            .fix_height(50.0);

        let mut inner_col = Flex::column();
        inner_col.add_flex_child(btn, 1.0);
        row.add_flex_child(inner_col, 1.0);
    }

    row
}

pub fn build_app_ui(state: &AppState) -> Box<dyn Widget<AppState>> {
    let mut col = Flex::column();

    col.set_main_axis_alignment(druid::widget::MainAxisAlignment::Start);

    let mut image_row = Flex::row();
    let image_folder_path = Path::new(&std::env::current_dir().unwrap())
        .to_path_buf()
        .join("src/images/1/");

    let mut histogram_row = Flex::row();
    if let Some(image_path) = state.selected_image.clone() {
        let selected_image = image::open(image_folder_path.join(image_path)).unwrap();

        let (width, height) = selected_image.get_dimensions();

        let make_sized = |inner: &DynamicImage| {
            SizedBox::new(
                inner
                    .to_druid_image()
                    .fill_mode(druid::widget::FillStrat::Cover),
            )
            .fix_width(width as f64 * 1.2)
            .fix_height(height as f64 * 1.2)
            .border(Color::grey(0.6), 2.0)
            .padding(Insets::uniform(10.0))
        };

        let original_image = make_sized(&selected_image);

        image_row.add_flex_child(original_image, 1.0);

        image_row.add_flex_child(exec_op(&selected_image, state), 1.0);
        histogram_row.add_flex_child(build_histogram(&selected_image, state), 1.0);
    };
    col.add_flex_child(build_image_list(), 1.0);
    col.add_flex_child(build_operation_list(), 1.0);
    col.add_flex_child(image_row, 4.0);
    col.add_flex_child(histogram_row, 1.0);
    col.boxed()
}

pub fn build_histogram(image: &DynamicImage, state: &AppState) -> impl Widget<AppState> {
    let build_image = || {
        SizedBox::new(
            apply_operation(image, state.selected_operation, &state)
                .render_grayscale_histogram()
                .to_druid_image()
                .fill_mode(druid::widget::FillStrat::Fill),
        )
        .fix_width(1024.0)
        .fix_height(512.0)
        .border(Color::grey(0.6), 2.0)
        .padding(Insets::uniform(10.0))
    };

    build_image()
}

pub fn exec_op(image: &DynamicImage, state: &AppState) -> impl Widget<AppState> {
    let (width, height) = image.get_dimensions();

    let build_image = |op: Operation, state: &AppState| {
        SizedBox::new(
            apply_operation(image, op, &state)
                .to_druid_image()
                .fill_mode(druid::widget::FillStrat::Cover),
        )
        .fix_width(width as f64 * 1.2)
        .fix_height(height as f64 * 1.2)
        .border(Color::grey(0.6), 2.0)
        .padding(Insets::uniform(10.0))
    };

    if state.selected_operation == Operation::Save {
        let image_to_save = apply_operation(image, state.last_operation, state);

        let result_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/result_images");

        let selected = state.selected_image.clone().unwrap();
        let image_name = Path::new(&selected).file_name().unwrap().to_str().unwrap();
        let format_save = |op| result_path.join(format!("{}-{}.jpg", op, image_name));

        match state.last_operation {
            Operation::FlipH => image_to_save.save(format_save("flip_h")).unwrap(),
            Operation::Negative => image_to_save.save(format_save("negative")).unwrap(),
            Operation::FlipV => image_to_save.save(format_save("flip_v")).unwrap(),
            Operation::Save => (),
            Operation::Grayscale => image_to_save.save(format_save("grayscale")).unwrap(),
            Operation::Quantize => image_to_save
                .save(format_save(&format!("quantize-{}", state.param1 as u8)))
                .unwrap(),
            Operation::AdjustBrightness => image_to_save
                .save(format_save(&format!("brightness-{}", state.param1 as u8)))
                .unwrap(),

            Operation::AdjustContrast => image_to_save
                .save(format_save(&format!("constrast-{}", state.param1 as u8)))
                .unwrap(),
            Operation::ZoomOut => image_to_save
                .save(format_save(&format!(
                    "zoomout-{}-{}",
                    state.param2.ceil() as u8,
                    state.param3.ceil() as u8
                )))
                .unwrap(),
            Operation::None => (),
        };
    }
    build_image(state.selected_operation, state)
}

pub fn apply_operation(image: &DynamicImage, op: Operation, state: &AppState) -> DynamicImage {
    // we dont want a stack overflow do we
    if state.last_operation == Operation::Save && state.selected_operation == Operation::Save {
        panic!("uh oh")
    }
    match op {
        Operation::FlipH => image.flip_h(),
        Operation::FlipV => image.flip_v(),
        Operation::Save => apply_operation(image, state.last_operation, state),
        Operation::Grayscale => image.to_grayscale_rgb(),
        Operation::Quantize => image.quantize_grayscale(state.param1 as u8),
        Operation::None => image.clone(),
        Operation::AdjustBrightness => image.adjust_brightness(state.param1 as u8),
        Operation::AdjustContrast => image.adjust_contrast_2(state.param1 as u8),
        Operation::Negative => image.negative(),
        Operation::ZoomOut => image.zoom_out(state.param2 as u8, state.param3 as u8),
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, path::Path};

    use crate::imageops::ImageExt;
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

    #[test]
    fn flip_v_naive_eq() -> Result<(), Box<dyn Error>> {
        let image_folder_path = Path::new(&std::env::current_dir().unwrap())
            .to_path_buf()
            .join("src/images/1/");

        let img = image::open(dbg!(image_folder_path.join("Gramado_22k.jpg")))?;

        let flip_v_fast = img.flip_v();
        let flip_v_naive = img.flipv();

        assert_eq!(flip_v_fast, flip_v_naive);
        Ok(())
    }
}
