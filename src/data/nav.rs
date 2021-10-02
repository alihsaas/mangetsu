use std::sync::Arc;

use druid::Data;
use serde::{Deserialize, Serialize};

use super::AppState;

#[derive(Clone, Debug, Data, PartialEq, Eq, Deserialize, Serialize)]
pub enum Nav {
    Home,
    Downloads,
    MangaPage(Arc<str>),
}

impl Nav {
    pub fn title(&self, data: &AppState) -> String {
        match self {
            Nav::Home => "Home".to_string(),
            Nav::Downloads => "Downloads".to_string(),
            Nav::MangaPage(manga) => data
                .manga_cache
                .lock()
                .unwrap()
                .get_mut(manga)
                .map(|manga| manga.title.clone())
                .unwrap_or_else(|| "Manga Not Found".into())
                .to_string(),
        }
    }

    pub fn full_title(&self, data: &AppState) -> String {
        match self {
            Nav::Home => "Home".to_string(),
            Nav::Downloads => "Downloads".to_string(),
            Nav::MangaPage(manga) => data
                .manga_cache
                .lock()
                .unwrap()
                .get_mut(manga)
                .map(|manga| format!("Manga - {} - {}", manga.title, manga.connector))
                .unwrap_or_else(|| "Manga Not Found".into()),
        }
    }
}
