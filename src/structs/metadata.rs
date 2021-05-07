use serde::Serialize;
use std::collections::HashMap;

use crate::util::{
    request::{consume_size, fetch},
    result::Error,
};

use super::media::{Media, MediaSize};

#[derive(Default, Debug, Serialize)]
pub struct Metadata {
    title: Option<String>,
    description: Option<String>,
    image: Option<Media>,
}

impl Metadata {
    pub fn from(mut properties: HashMap<String, String>) -> Metadata {
        Metadata {
            title: properties
                .remove("og:title")
                .or_else(|| properties.remove("twitter:title"))
                .or_else(|| properties.remove("title")),
            description: properties
                .remove("og:description")
                .or_else(|| properties.remove("twitter:description"))
                .or_else(|| properties.remove("description")),
            image: properties
                .remove("og:image")
                .or_else(|| properties.remove("twitter:image"))
                .map(|url| {
                    let mut size = MediaSize::Preview;
                    if let Some(card) = properties.remove("twitter:card") {
                        if &card == "summary_large_image" {
                            size = MediaSize::Large;
                        }
                    }

                    Media {
                        url,
                        width: 0,
                        height: 0,
                        size,
                    }
                }),
        }
    }

    async fn resolve_image(&mut self) -> Result<(), Error> {
        if let Some(image) = &mut self.image {
            let (resp, _) = fetch(&image.url).await?;
            let (width, height) = consume_size(resp).await?;

            image.width = width;
            image.height = height;
        }

        Ok(())
    }

    pub async fn resolve_external(&mut self) {
        if self.resolve_image().await.is_err() {
            self.image = None;
        }
    }
}
