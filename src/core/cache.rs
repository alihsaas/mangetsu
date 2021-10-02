use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use druid::{image::RgbImage, ImageBuf};

pub fn mkdir_if_not_exists(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path).or_else(|err| {
        if err.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(err)
        }
    })
}

#[derive(Debug)]
pub struct Cache {
    base: Option<PathBuf>,
}

impl Cache {
    pub fn new(base: Option<PathBuf>) -> Self {
        Self { base }
    }

    pub fn get(&self, bucket: &str, key: &str) -> Option<File> {
        self.key(bucket, key).and_then(|path| File::open(path).ok())
    }

    pub fn set(&self, bucket: &str, key: &str, value: &[u8]) {
        if let Some(path) = self.bucket(bucket) {
            if let Err(err) = mkdir_if_not_exists(&path) {
                log::error!("failed to create cache bucket: {:?}", err);
            }
        }
        if let Some(path) = self.key(bucket, key) {
            if let Err(err) = fs::write(path, value) {
                log::error!("failed to save to cache: {:?}", err);
            }
        }
    }

    pub fn set_image(&self, bucket: &str, key: &str, image: &ImageBuf) {
        if let Some(path) = self.bucket(bucket) {
            if let Err(err) = mkdir_if_not_exists(&path) {
                log::error!("failed to create cache bucket: {:?}", err);
            }
        }
        if let Some(image) = RgbImage::from_raw(
            image.width() as u32,
            image.height() as u32,
            image.raw_pixels().into(),
        ) {
            if let Some(path) = self.key(bucket, key) {
                if let Err(err) = image.save(path) {
                    log::error!("failed to save to cache: {:?}", err);
                };
            }
        };
    }

    pub fn get_image(&self, bucket: &str, key: &str) -> Option<ImageBuf> {
        self.key(bucket, key)
            .and_then(|path| ImageBuf::from_file(path).ok())
    }

    fn bucket(&self, bucket: &str) -> Option<PathBuf> {
        self.base.as_ref().map(|path| path.join(bucket))
    }

    fn key(&self, bucket: &str, key: &str) -> Option<PathBuf> {
        self.bucket(bucket).map(|path| path.join(key))
    }
}
