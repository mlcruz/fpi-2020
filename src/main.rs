use druid::widget::prelude::*;
use druid::{AppLauncher, WindowDesc};
use fpi::{imageops::Operation, AppState, UiBuilder};

fn make_ui() -> impl Widget<AppState> {
    UiBuilder::new()
}

pub fn main() {
    let main_window = WindowDesc::new(|| make_ui())
        .window_size((800., 600.))
        .title("Fpi - 2020 - Matheus Leite Cruz");

    let state = AppState {
        selected_image: "Gramado_22k.jpg".to_owned().into(),
        selected_operation: Operation::None,
        param1: 64.0,
        param2: 2.0,
        param3: 2.0,
        last_operation: Operation::None,
    };

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
