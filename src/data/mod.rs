pub mod cmd;
mod nav;

use std::sync::Arc;

use druid::{
    im::{vector, Vector},
    Data, Lens, WindowState,
};

pub use nav::Nav;
use reqwest::Url;

use crate::core::Manga;

#[derive(Data, Lens, Clone)]
pub struct AppState {
    pub theme: Theme,
    pub route: Nav,
    pub manga_search_url: String,
    pub mangas: Vector<Manga>,
    pub window_state: Arc<WindowState>,
}

impl AppState {
    pub fn navigate(&mut self, nav: &Nav) {
        if &self.route != nav {
            let _previous = std::mem::replace(&mut self.route, nav.to_owned());
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let manga = Manga {
            title: "Hello World".to_string(),
            url: Url::parse("https://manganel.com").unwrap(),
            icon_url: Url::parse("https://manganel.com").unwrap(),
            connector: crate::core::Connectors::Manganel,
        };
        Self {
            theme: Default::default(),
            route: Nav::Home,
            manga_search_url: Default::default(),
            mangas: Default::default(),
            window_state: Arc::new(WindowState::MAXIMIZED),
        }
    }
}

#[derive(Data, Clone, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}
