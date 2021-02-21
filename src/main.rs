use druid::widget::prelude::*;
use druid::{AppLauncher, WindowDesc};
use fpi::{AppState, UiBuilder};

fn make_ui() -> impl Widget<AppState> {
    UiBuilder::new()
}

pub fn main() {
    let main_window = WindowDesc::new(|| make_ui())
        .window_size((800., 600.))
        .title("Flex Container Options");

    let state = AppState {
        selected_image: "Gramado_22k.jpg".to_owned().into(),
        selected_operation: fpi::Operation::FlipV,
    };

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
