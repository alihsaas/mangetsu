use druid::{Data, Lens};
use reqwest::Url;

use crate::core::Connectors;

#[derive(Clone, Debug, Eq, Lens)]
pub struct Manga {
    pub url: Url,
    pub title: String,
    pub icon_url: Url,
    pub connector: Connectors,
}

impl PartialEq for Manga {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(&other.url)
    }
}

impl Data for Manga {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
