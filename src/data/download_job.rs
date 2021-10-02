use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use druid::{Data, ExtEventSink, Lens, Target};
use futures::StreamExt;
use reqwest::Response;
use reqwest::{header::REFERER, Url};
use verbatim::PathExt;

use crate::core::cache::mkdir_if_not_exists;
use crate::core::error::map_to_string;
use crate::core::{error::Error, Chapter, GlobalAPI};
use crate::data::cmd;

use super::Config;

#[derive(Data, Clone, Lens)]
pub struct DownloadJob {
    pub chapter: Chapter,
    pub progress: f64,
}

impl DownloadJob {
    pub fn new(chapter: Chapter) -> Self {
        Self {
            chapter,
            progress: 0.,
        }
    }

    pub fn with_progress(chapter: Chapter, progress: f64) -> Self {
        Self { chapter, progress }
    }
}

fn to_verbatim(path: &Path) -> PathBuf {
    if cfg!(target_os = "windows") {
        path.to_verbatim()
    } else {
        path.to_owned()
    }
}

fn download_path(chapter: &Chapter) -> Result<PathBuf, Error> {
    if let Some(download_dir) = Config::download_dir() {
        let manga_path =
            download_dir.join(&sanitize_filename::sanitize(chapter.manga.title.as_ref()));
        let chapter_path =
            to_verbatim(&manga_path.join(&sanitize_filename::sanitize(chapter.title.as_ref())));
        mkdir_if_not_exists(&chapter_path).map_err(map_to_string(Error::IoError))?;
        File::create(manga_path.join("metadata.json"))
            .and_then(|mut file| file.write(&serde_json::to_vec_pretty(&chapter.manga).unwrap()))
            .map_err(map_to_string(Error::IoError))?;
        File::create(chapter_path.join("metadata.json"))
            .and_then(|mut file| file.write(&serde_json::to_vec_pretty(chapter).unwrap()))
            .map_err(map_to_string(Error::IoError))?;
        Ok(chapter_path)
    } else {
        Err(Error::IoError("Failed to get Download Path".to_string()))
    }
}

pub async fn start_download(chapter: &Chapter, event_sink: ExtEventSink) -> Result<(), Error> {
    let pages = chapter.get_pages().enumerate().collect::<Vec<_>>().await;
    let mut final_chunk_size = 0.;
    let mut chunk_progress = 0.;
    for (index, res) in &pages {
        if let Ok(page) = res {
            let download_path = download_path(chapter)?;
            let page_name = Path::new(page.url.as_ref())
                .extension()
                .map(|extention| format!("{}.{}", index, extention.to_string_lossy().as_ref()))
                .expect("Invalid Page URL");
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(download_path.join(page_name))
                .map_err(map_to_string(Error::IoError))?;
            let mut request: Response = GlobalAPI::global()
                .client
                .get(Url::parse(&page.url).unwrap())
                .header(REFERER, page.referer.as_ref())
                .send()
                .await
                .map_err(map_to_string(Error::RequestFail))?;
            final_chunk_size += request
                .content_length()
                .map(|content| content as f64)
                .unwrap_or(0.);
            while let Some(chunk) = request
                .chunk()
                .await
                .map_err(map_to_string(Error::IoError))?
            {
                chunk_progress += chunk.len() as f64;
                let progress = chunk_progress / final_chunk_size * ((*index as f64) + 1.) * 100.
                    / pages.len() as f64
                    / 100.;
                event_sink
                    .submit_command(
                        cmd::UPDATE_DOWNLOAD_PROGRESS,
                        (chapter.clone(), progress),
                        Target::Auto,
                    )
                    .unwrap();
                file.write_all(&chunk.to_vec()).unwrap();
            }
        }
    }

    Ok(())
}
