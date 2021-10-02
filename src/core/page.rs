use std::sync::Arc;

use crate::core::Connectors;

#[derive(Clone, Debug)]
pub struct Page {
    pub url: Arc<str>,
    pub referer: Arc<str>,
    pub connector: Connectors,
}
