mod app;
mod manga;
pub mod theme;

use druid::Env;

pub use app::app_widget;

use crate::data::AppState;

pub fn compute_window_title(data: &AppState, _env: &Env) -> String {
    format!("Mangetsu - [{}]", data.route.full_title())
}
