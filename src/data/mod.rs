pub mod cmd;
mod config;
mod download_job;
mod nav;

use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

use druid::{
    im::{vector, Vector},
    widget::ListIter,
    Data, Lens, WindowState,
};
use indexmap::IndexMap;
use lru_cache::LruCache;

pub use config::{Config, Theme};
pub use download_job::{start_download, DownloadJob};
pub use nav::Nav;

use crate::core::{Chapter, Manga};

#[derive(Data, Lens, Clone)]
pub struct MangaDetail {
    pub manga: Manga,
    pub start: u16,
    pub end: u16,
    pub chapters: Vector<Chapter>,
}

#[derive(Data, Lens, Clone)]
pub struct AppState {
    pub config: Config,
    pub route: Nav,
    pub manga_detail: Option<MangaDetail>,
    pub mangas: Vector<Manga>,
    pub manga_cache: Arc<Mutex<LruCache<Arc<str>, Manga>>>,
    pub manga_chapters_cache: Arc<Mutex<LruCache<Arc<str>, Vec<Manga>>>>,
    pub manga_search_url: String,
    pub download_queue: MyIndexMap<Arc<str>, Vector<DownloadJob>>,
    pub window_state: Arc<WindowState>,
}

impl AppState {
    pub fn navigate(&mut self, nav: &Nav) {
        if &self.route != nav {
            let _previous = std::mem::replace(&mut self.route, nav.to_owned());
            self.config.last_route.replace(nav.to_owned());
            self.config.save();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let _chapter = Chapter {
            title: "Hello WORRRRRLD OMMMGGGGGGG".into(),
            url: "Hello".into(),
            connector: crate::core::Connectors::Manganel,
            manga: Manga {
                title: "Hello".into(),
                url: "Hello".into(),
                icon_url: "Hello".into(),
                connector: crate::core::Connectors::Manganel,
            },
        };
        Self {
            config: Config::load().unwrap_or_default(),
            route: Nav::Home,
            manga_detail: None,
            mangas: vector![],
            manga_cache: Arc::new(Mutex::new(LruCache::new(256))),
            manga_chapters_cache: Arc::new(Mutex::new(LruCache::new(256))),
            manga_search_url: Default::default(),
            download_queue: MyIndexMap(IndexMap::new()),
            window_state: Arc::new(WindowState::MAXIMIZED),
        }
    }
}

#[derive(Clone)]
pub struct MyIndexMap<K, V>(pub IndexMap<K, V>);

impl<K: Data + Eq + Hash, T: Data> ListIter<T> for MyIndexMap<K, T> {
    fn for_each(&self, mut cb: impl FnMut(&T, usize)) {
        for (i, item) in self.0.iter().enumerate() {
            cb(item.1, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut T, usize)) {
        let mut new_data = IndexMap::with_capacity(self.data_len());
        let mut any_changed = false;

        for (i, (k, item)) in self.0.iter().enumerate() {
            let mut d = item.to_owned();
            cb(&mut d, i);

            if !any_changed && !item.same(&d) {
                any_changed = true;
            }
            new_data.insert(k.clone(), d);
        }

        if any_changed {
            *self = MyIndexMap(new_data);
        }
    }

    fn data_len(&self) -> usize {
        self.0.len()
    }
}

impl<K: Data + Eq + Hash, T: Data> Data for MyIndexMap<K, T> {
    fn same(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
