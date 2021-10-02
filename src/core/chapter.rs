use std::sync::Arc;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::core::Connectors;

use super::{connector::StreamResult, GlobalAPI, Manga, Page};

#[derive(Clone, Debug, Lens, Deserialize, Serialize)]
pub struct Chapter {
    pub url: Arc<str>,
    pub title: Arc<str>,
    pub connector: Connectors,
    pub manga: Manga,
}

impl Chapter {
    pub fn get_pages(&self) -> StreamResult<Page> {
        GlobalAPI::global()
            .connectors
            .get(&self.connector)
            .unwrap()
            .get_pages(self.clone())
    }
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
