use std::{path::PathBuf, sync::Arc};

use indexmap::{indexmap, IndexMap};
use once_cell::sync::OnceCell;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use crate::core::{cache::Cache, websites::manganel::Manganel, Connector};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Connectors {
    Manganel,
}

impl std::fmt::Display for Connectors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let connector = GlobalAPI::global()
            .connectors
            .get(self)
            .expect("Connector Not Found");
        f.write_str(connector.get_connector_info().label)
    }
}

static GLOBAL_API: OnceCell<Arc<GlobalAPI>> = OnceCell::new();

type Value = Box<dyn Connector + Send + Sync>;

#[derive(Debug)]
pub struct GlobalAPI {
    pub connectors: IndexMap<Connectors, Value>,
    pub client: Client,
    pub cache: Cache,
}

impl GlobalAPI {
    pub fn install(cache_base: Option<PathBuf>) {
        let connectors = indexmap! {
            Connectors::Manganel => Box::new(Manganel::new()) as Value,
        };

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());

        GLOBAL_API
            .set(Arc::new(GlobalAPI {
                connectors,
                client: Client::builder().default_headers(headers).build().unwrap(),
                cache: Cache::new(cache_base),
            }))
            .unwrap();
    }

    pub fn global<'a>() -> &'a GlobalAPI {
        GLOBAL_API
            .get()
            .expect("GlobalAPI Not Initialized! Did you forget to call GlobalAPI::setup?")
    }
}
