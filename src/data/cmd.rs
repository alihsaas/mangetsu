use std::sync::Arc;

use druid::Selector;

use crate::core::{Chapter, Manga};

use super::Nav;

pub const NAVIGATE: Selector<Nav> = Selector::new("app.navigates");

pub const DOWNLOAD_CHAPTER: Selector<Chapter> = Selector::new("app.download-chapter");
pub const UPDATE_DOWNLOAD_PROGRESS: Selector<(Chapter, f64)> =
    Selector::new("app.update-download-progress");
pub const START_DOWNLOAD: Selector = Selector::new("app.start-download");
pub const POP_QUEUE: Selector = Selector::new("app.pop-queue");

pub const FETCH_MANGA_DETAIL: Selector<Arc<str>> = Selector::new("app.fetch-manga-detail");
pub const LOAD_MANGA_DETAIL: Selector<Manga> = Selector::new("app.load-manga-detail");
