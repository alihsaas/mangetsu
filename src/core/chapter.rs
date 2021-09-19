use reqwest::Url;

use crate::core::Connectors;

#[derive(Clone, Debug)]
pub struct Chapter {
    pub url: Url,
    pub title: String,
    pub language: String,
    pub connector: Connectors,
}
