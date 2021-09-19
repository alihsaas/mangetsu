use reqwest::Url;

use crate::core::Connectors;

#[derive(Clone, Debug)]
pub struct Page {
    pub url: Url,
    pub referer: Url,
    pub connector: Connectors,
}
