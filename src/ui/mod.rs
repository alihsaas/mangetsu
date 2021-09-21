mod app;
mod chapter;
mod manga;
pub mod theme;

use druid::{Data, Env, LocalizedString, MenuDesc};

pub use app::app_widget;

use crate::data::AppState;

pub fn compute_window_title(data: &AppState, _env: &Env) -> String {
    format!("Mangetsu - [{}]", data.route.full_title())
}

#[allow(unused_assignments, unused_mut)]
pub fn make_menu<T: Data>() -> MenuDesc<T> {
    let mut base = MenuDesc::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.append(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.append(druid::platform_menus::win::file::default());
    }
    base.append(
        MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
            .append(druid::platform_menus::common::undo())
            .append(druid::platform_menus::common::redo())
            .append_separator()
            .append(druid::platform_menus::common::cut())
            .append(druid::platform_menus::common::copy())
            .append(druid::platform_menus::common::paste()),
    )
}
