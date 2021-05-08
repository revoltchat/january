use reqwest::Response;
use scraper::Selector;
use serde::Serialize;
use std::collections::HashMap;

use crate::util::{
    request::{consume_fragment, consume_size, fetch},
    result::Error,
};

use super::media::{Media, MediaSize};

#[derive(Debug, Serialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Media>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    site_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl Metadata {
    pub async fn from(resp: Response, url: String) -> Result<Metadata, Error> {
        let fragment = consume_fragment(resp).await?;

        let meta_selector = Selector::parse("meta").map_err(|_| Error::MetaSelectionFailed)?;
        let mut meta = HashMap::new();
        for el in fragment.select(&meta_selector) {
            let node = el.value();

            if let (Some(property), Some(content)) = (
                node.attr("property").or(node.attr("name")),
                node.attr("content"),
            ) {
                meta.insert(property.to_string(), content.to_string());
            }
        }

        let link_selector = Selector::parse("link").map_err(|_| Error::MetaSelectionFailed)?;
        let mut link = HashMap::new();
        for el in fragment.select(&link_selector) {
            let node = el.value();

            if let (Some(property), Some(content)) = (node.attr("rel"), node.attr("href")) {
                link.insert(property.to_string(), content.to_string());
            }
        }

        Ok(Metadata {
            title: meta
                .remove("og:title")
                .or_else(|| meta.remove("twitter:title"))
                .or_else(|| meta.remove("title")),
            description: meta
                .remove("og:description")
                .or_else(|| meta.remove("twitter:description"))
                .or_else(|| meta.remove("description")),
            image: meta
                .remove("og:image")
                .or_else(|| meta.remove("og:image:secure_url"))
                .or_else(|| meta.remove("twitter:image"))
                .or_else(|| meta.remove("twitter:image:src"))
                .map(|url| {
                    let mut size = MediaSize::Preview;
                    if let Some(card) = meta.remove("twitter:card") {
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
            icon_url: link
                .remove("apple-touch-icon")
                .or_else(|| link.remove("icon"))
                .map(|mut v| {
                    // If relative URL, prepend root URL.
                    if let Some(ch) = v.chars().nth(0) {
                        if ch == '/' {
                            v = format!("{}{}", &url, v);
                        }
                    }

                    v
                }),
            color: meta.remove("theme-color"),
            site_name: meta.remove("og:site_name"),
            url: meta.remove("og:url").or(Some(url)),
        })
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

    pub fn is_none(&self) -> bool {
        self.title.is_none() && self.description.is_none() && self.image.is_none()
    }
}
