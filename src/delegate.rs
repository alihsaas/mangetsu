use std::{
    io::{copy, Cursor},
    path::Path,
    sync::Arc,
};

use druid::{
    im::Vector,
    image::{self, ImageFormat},
    AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, ImageBuf, Target,
};
use lru_cache::LruCache;
use reqwest::header::{self, CONTENT_TYPE};

use crate::{
    core::GlobalAPI,
    data::{cmd, start_download, AppState, DownloadJob},
    widgets::remote_image,
};

pub struct Delegate {
    image_cache: LruCache<Arc<str>, ImageBuf>,
    event_sink: ExtEventSink,
}

impl Delegate {
    pub fn new(event_sink: ExtEventSink) -> Self {
        const IMAGE_CACHE_SIZE: usize = 256;
        let image_cache = LruCache::new(IMAGE_CACHE_SIZE);

        Self {
            image_cache,
            event_sink,
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Handled::Yes = self.command_image(ctx, target, cmd, data) {
            Handled::Yes
        } else {
            self.command_download(ctx, target, cmd, data)
        }
    }
}

#[derive(Debug)]
enum ImageRequestError {
    RequestError(String),
    ImageLoadError(String),
}

async fn get_image(
    uri: &str,
    referer: Option<&str>,
) -> Result<image::DynamicImage, ImageRequestError> {
    let response = GlobalAPI::global()
        .client
        .get(uri)
        .header(header::REFERER, referer.unwrap_or(uri))
        .send()
        .await
        .map_err(|err| ImageRequestError::RequestError(err.to_string()))?;
    let format = match response
        .headers()
        .get(CONTENT_TYPE)
        .map(|v| v.to_str().unwrap())
    {
        Some("image/jpeg") => Some(ImageFormat::Jpeg),
        Some("image/png") => Some(ImageFormat::Png),
        _ => None,
    };
    let mut content = Cursor::new(
        response
            .bytes()
            .await
            .map_err(|err| ImageRequestError::RequestError(err.to_string()))?,
    );
    let mut body = vec![];
    copy(&mut content, &mut body).unwrap();
    let image = if let Some(format) = format {
        image::load_from_memory_with_format(&body, format)
            .map_err(|err| ImageRequestError::ImageLoadError(err.to_string()))?
    } else {
        image::load_from_memory(&body)
            .map_err(|err| ImageRequestError::ImageLoadError(err.to_string()))?
    };
    Ok(image)
}

impl Delegate {
    fn command_image(
        &mut self,
        _ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        _data: &mut AppState,
    ) -> Handled {
        if let Some(location) = cmd.get(remote_image::REQUEST_DATA).cloned() {
            log::info!("Reqesting Image");
            let image_key = Path::new(location.as_ref()).file_name().unwrap().to_owned();
            let cache = &GlobalAPI::global().cache;
            if let Some(image_buf) = self.image_cache.get_mut(&location).cloned() {
                log::info!("Memory Image");
                let payload = remote_image::ImagePayload {
                    location,
                    image_buf,
                };

                self.event_sink
                    .submit_command(remote_image::PROVIDE_DATA, payload, target)
                    .expect("Command failed to submit");
            } else if let Some(image_buf) =
                cache.get_image("images", image_key.to_string_lossy().as_ref())
            {
                log::info!("Old Image");
                let payload = remote_image::ImagePayload {
                    location,
                    image_buf,
                };

                self.event_sink
                    .submit_command(remote_image::PROVIDE_DATA, payload, target)
                    .expect("Command failed to submit");
            } else {
                log::info!("Grabbing Image");
                let event_sink = self.event_sink.clone();
                tokio::spawn(async move {
                    let image_buf: ImageBuf = {
                        let dyn_image = get_image(&location, None).await.unwrap();
                        ImageBuf::from_dynamic_image(dyn_image)
                    };
                    cache.set_image("images", image_key.to_string_lossy().as_ref(), &image_buf);
                    let payload = remote_image::ImagePayload {
                        location,
                        image_buf,
                    };
                    event_sink
                        .submit_command(remote_image::PROVIDE_DATA, payload, target)
                        .expect("Command failed to submit");
                });
            }
            Handled::Yes
        } else if let Some(payload) = cmd.get(remote_image::PROVIDE_DATA).cloned() {
            self.image_cache.insert(payload.location, payload.image_buf);
            Handled::No
        } else {
            Handled::No
        }
    }

    fn command_download(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
    ) -> Handled {
        if let Some(chapter) = cmd.get(cmd::DOWNLOAD_CHAPTER).cloned() {
            let download_job = DownloadJob::new(chapter.clone());
            data.download_queue
                .0
                .entry(chapter.manga.url)
                .or_insert(Vector::new())
                .push_back(download_job);

            if data.download_queue.0.first().unwrap().1.len() == 1 {
                self.event_sink
                    .submit_command(cmd::START_DOWNLOAD, (), Target::Auto)
                    .unwrap();
            };
            Handled::Yes
        } else if let Some(()) = cmd.get(cmd::START_DOWNLOAD) {
            if let Some((_, download_queue)) = data.download_queue.0.first() {
                if let Some(download_job) = download_queue.get(0).cloned() {
                    let event_sink = self.event_sink.clone();
                    tokio::spawn(async move {
                        log::info!(
                            "Starting download of {}",
                            download_job.chapter.title.as_ref()
                        );
                        if start_download(&download_job.chapter, event_sink.clone())
                            .await
                            .is_ok()
                        {
                            event_sink
                                .submit_command(cmd::POP_QUEUE, (), Target::Auto)
                                .unwrap();
                            event_sink
                                .submit_command(cmd::START_DOWNLOAD, (), Target::Auto)
                                .unwrap();
                        };
                    });
                }
            };
            Handled::Yes
        } else if let Some(()) = cmd.get(cmd::POP_QUEUE) {
            if let Some((_, download_queue)) = data.download_queue.0.first_mut() {
                download_queue.pop_front();
                if download_queue.is_empty() {
                    data.download_queue.0.pop();
                }
            }
            Handled::Yes
        } else if let Some((chapter, progress)) = cmd.get(cmd::UPDATE_DOWNLOAD_PROGRESS).cloned() {
            let manga_download_queue = &mut data.download_queue.0;
            if let Some((_, download_queue)) = manga_download_queue.first() {
                let (index, previous_download_job) = download_queue
                    .iter()
                    .enumerate()
                    .find(|(_index, download_job)| download_job.chapter.url == chapter.url)
                    .unwrap();
                let new_download_queue = download_queue.update(
                    index,
                    DownloadJob::with_progress(previous_download_job.chapter.clone(), progress),
                );
                manga_download_queue.insert(chapter.manga.url, new_download_queue);
            }
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
