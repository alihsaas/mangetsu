mod chapter;
mod connector;
pub mod error;
mod global_api;
mod manga;
mod page;
mod websites;

pub use chapter::Chapter;
pub use connector::{Connector, ConnectorInfo};
pub use global_api::{Connectors, GlobalAPI};
pub use manga::Manga;
pub use page::Page;
