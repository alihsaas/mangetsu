use std::fmt;

use futures::{future::BoxFuture, stream::BoxStream};
use regex::Regex;
use reqwest::Url;
use scraper::Selector;

use crate::core::{error::Error, Chapter, Manga, Page};

#[derive(Debug, Clone)]
pub struct ConnectorInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub tags: Vec<&'static str>,
    pub url: Url,

    pub path: &'static str,
    pub manga_title_filter: Regex,
    pub chapter_title_filter: Regex,
    pub query_manga_title: Selector,
    pub query_mangas_page_count: Selector,
    pub query_mangas: Selector,

    pub query_icon: Selector,

    pub query_chapters: Selector,
    pub query_pages: Selector,
}

pub type StreamResult<'a, T> = BoxStream<'a, Result<T, Error>>;
pub type FutureResult<'a, T> = BoxFuture<'a, Result<T, Error>>;

pub trait Connector {
    fn get_connector_info(&self) -> ConnectorInfo;

    fn can_handle_uri(&self, uri: Url) -> bool;

    fn get_manga_from_url(&self, manga_url: Url) -> FutureResult<Manga>;

    fn get_mangas(&self) -> StreamResult<Manga>;

    fn get_manga_icon(&self, manga_url: Url) -> FutureResult<Url>;

    fn get_mangas_from_page(&self, page: i32) -> StreamResult<Manga>;

    fn get_chapters(&self, manga: Manga) -> StreamResult<Chapter>;

    fn get_pages(&self, chapter: Chapter) -> StreamResult<Page>;
}

impl fmt::Debug for dyn Connector + Sync + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let connector_info = self.get_connector_info();
        f.debug_struct("Connector")
            .field("Name", &connector_info.label)
            .field("Url", &connector_info.url.as_str())
            .finish()
    }
}
