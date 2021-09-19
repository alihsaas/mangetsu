use druid::Data;

use crate::core::Manga;

#[derive(Clone, Debug, Data, PartialEq, Eq)]
pub enum Nav {
    Home,
    Downloads,
    MangaPage(Manga),
}

impl Nav {
    pub fn title(&self) -> String {
        match self {
            Nav::Home => "Home".to_string(),
            Nav::Downloads => "Downloads".to_string(),
            Nav::MangaPage(manga) => manga.title.to_string(),
        }
    }

    pub fn full_title(&self) -> String {
        match self {
            Nav::Home => "Home".to_string(),
            Nav::Downloads => "Downloads".to_string(),
            Nav::MangaPage(manga) => format!("Manga - {} - {}", manga.title, manga.connector),
        }
    }
}
