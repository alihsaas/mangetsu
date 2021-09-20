use std::{
    io::{copy, Cursor},
    sync::Arc,
};

use druid::{
    image::{self, ImageFormat},
    AppDelegate, Command, DelegateCtx, Env, ExtEventSink, Handled, ImageBuf, Target,
};
use lru_cache::LruCache;
use reqwest::header::{self, CONTENT_TYPE};

use crate::{core::GlobalAPI, data::AppState, widgets::remote_image};

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
        self.command_image(ctx, target, cmd, data)
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
            if let Some(image_buf) = self.image_cache.get_mut(&location).cloned() {
                let payload = remote_image::ImagePayload {
                    location,
                    image_buf,
                };

                self.event_sink
                    .submit_command(remote_image::PROVIDE_DATA, payload, target)
                    .expect("Command failed to submit");
            } else {
                let event_sink = self.event_sink.clone();
                tokio::spawn(async move {
                    let image_buf = {
                        let dyn_image = get_image(&location, None).await.unwrap();
                        ImageBuf::from_dynamic_image(dyn_image)
                    };
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
}
