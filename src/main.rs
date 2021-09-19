mod controller;
mod core;
mod data;
mod delegate;
mod ui;
mod widgets;

use druid::{AppLauncher, WindowDesc};

use crate::core::GlobalAPI;
use data::AppState;
use delegate::Delegate;
use ui::{app_widget, compute_window_title, theme};
use widgets::AsyncDelegate;

#[tokio::main]
async fn main() {
    GlobalAPI::install();

    let state = AppState::default();

    let main_window = WindowDesc::new(app_widget)
        .title(compute_window_title)
        .with_min_size((theme::grid(65.0), theme::grid(25.0)))
        .window_size((theme::grid(70.0), theme::grid(70.0)))
        .set_window_state(*state.window_state)
        .show_titlebar(false);
    // Set our initial data
    let app = AppLauncher::with_window(main_window);

    let delegate = Delegate::new(app.get_external_handle());

    AsyncDelegate::with_delegate(app, delegate)
        .configure_env(ui::theme::setup)
        //.use_simple_logger()
        .launch(state)
        .expect("launch failed")
}
