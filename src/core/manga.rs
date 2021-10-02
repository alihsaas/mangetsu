use std::sync::Arc;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use super::{connector::StreamResult, Chapter, Connectors, GlobalAPI};

#[derive(Clone, Debug, Eq, Lens, Deserialize, Serialize)]
pub struct Manga {
    pub url: Arc<str>,
    pub title: Arc<str>,
    pub icon_url: Arc<str>,
    pub connector: Connectors,
}

impl Manga {
    pub fn get_chapters(&self) -> StreamResult<Chapter> {
        GlobalAPI::global()
            .connectors
            .get(&self.connector)
            .unwrap()
            .get_chapters(self.clone())
    }
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
