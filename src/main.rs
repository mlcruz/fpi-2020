use std::{collections::HashMap, path::Path};

use druid::widget::prelude::*;
use druid::widget::{Flex, SizedBox, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, WindowDesc};
use fpi::{AppState, Rebuilder, ToDruidImage};

fn make_ui() -> impl Widget<AppState> {
    Flex::column().with_flex_child(Rebuilder::new().center(), 1.0)
}

pub fn main() {
    let main_window = WindowDesc::new(|| make_ui())
        .window_size((650., 450.))
        .title("Flex Container Options");

    let state = AppState {
        selected_image: "Gramado_22k.jpg".to_owned().into(),
        selected_operation: fpi::Operation::FlipV,
    };

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
