use druid::{Data, Lens};
use reqwest::Url;

use crate::core::Connectors;

#[derive(Clone, Debug, Lens)]
pub struct Chapter {
    pub url: Url,
    pub title: String,
    pub language: String,
    pub connector: Connectors,
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(&other.url)
    }
}

impl Data for Chapter {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
