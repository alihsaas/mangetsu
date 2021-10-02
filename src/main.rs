mod controller;
mod core;
mod data;
mod delegate;
mod ui;
mod widgets;

use druid::{AppLauncher, WindowDesc};
use log::{Level, LevelFilter, Metadata, SetLoggerError};

use crate::core::GlobalAPI;
use data::{AppState, Config};
use delegate::Delegate;
use ui::{app_widget, compute_window_title, make_menu, theme};
use widgets::AsyncDelegate;

struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }
    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger)).map(|()| log::set_max_level(LevelFilter::Info))
}

#[tokio::main]
async fn main() {
    GlobalAPI::install(Config::cache_dir());

    let state = AppState::default();

    let main_window = WindowDesc::new(app_widget)
        .title(compute_window_title)
        .menu(make_menu())
        .with_min_size((theme::grid(100.0), theme::grid(80.0)))
        .set_window_state(*state.window_state)
        .show_titlebar(false);
    // Set our initial data
    let app = AppLauncher::with_window(main_window);

    let delegate = Delegate::new(app.get_external_handle());

    init().expect("Failed to setup logger");

    AsyncDelegate::with_delegate(app, delegate)
        .configure_env(ui::theme::setup)
        .launch(state)
        .expect("launch failed")
}
