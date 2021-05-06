use std::collections::HashMap;

use actix_web::{
    web::{self, Query},
    App, HttpRequest, HttpServer, Responder,
};
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub mod util;
use util::result::Error;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/embed", web::get().to(embed))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

#[derive(Deserialize)]
struct Parameters {
    url: String,
}

#[derive(Debug, Serialize)]
struct Media {
    url: String,
    width: isize,
    height: isize
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum Image {
    Preview(Media),
    Large(Media)
}

#[derive(Default, Debug, Serialize)]
struct Metadata {
    title: Option<String>,
    description: Option<String>,
    image: Option<Image>
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum Embed {
    Website(Metadata),
    Image(Media),
    None
}

async fn embed(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;

    let resp = reqwest::get(&url).await.map_err(|_| Error::LabelMe)?;
    if !resp.status().is_success() {
        return Err(Error::LabelMe);
    }

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or_else(|| Error::LabelMe)?
        .to_str()
        .map_err(|_| Error::LabelMe)?;
    
    let mine: mime::Mime = content_type.parse().map_err(|_| Error::LabelMe)?;

    if let mime::HTML = mine.subtype() {
        let body = resp.text().await.map_err(|_| Error::LabelMe)?;
        let fragment = Html::parse_document(&body);
        let selector = Selector::parse("meta").map_err(|_| Error::LabelMe)?;

        let mut properties = HashMap::<&str, &str>::new();
        for el in fragment.select(&selector) {
            let node = el.value();

            if let (Some(property), Some(content)) = (node.attr("property"), node.attr("content")) {
                properties.insert(property, content);
            }
        }

        let mut metadata = Metadata::default();
        metadata.title = properties
            .remove("og:title")
            .or_else(|| properties.remove("twitter:title"))
            .or_else(|| properties.remove("title"))
            .map(|v| v.to_string());
        
        metadata.description = properties
            .remove("og:description")
            .or_else(|| properties.remove("twitter:description"))
            .or_else(|| properties.remove("description"))
            .map(|v| v.to_string());
        
        metadata.image = properties.remove("og:image")
            .or_else(|| properties.remove("twitter:image"))
            .map(|url| {
                let url = url.to_string();
                let media = Media { url, width: 0, height: 0 };

                match properties.remove("twitter:card") {
                    Some("summary_large_image") => Image::Large(media),
                    _ => Image::Preview(media)
                }
            });

        if let Some(image) = &metadata.image {
            if let Ok(image) = match image {
                Image::Large( Media { url, .. } ) => fetch_media(MediaType::Large, url.to_string()),
                Image::Preview( Media { url, .. } ) => fetch_media(MediaType::Preview, url.to_string())
            }.await {
                metadata.image = Some(image);
            } else {
                metadata.image = None;
            }
        }
        
        Ok(web::Json(Embed::Website(metadata)))
    } else if let mime::IMAGE = mine.type_() {
        let bytes = resp.bytes().await.map_err(|_| Error::LabelMe)?;
        if let Ok(size) = imagesize::blob_size(&bytes) {
            let imagesize::ImageSize { width, height } = size;
            let  width =  width as isize;
            let height = height as isize;

            let media = Media { url, width, height };
            Ok(web::Json(Embed::Image(media)))
        } else {
            Ok(web::Json(Embed::None))
        }
    } else {
        Ok(web::Json(Embed::None))
    }
}

enum MediaType {
    Large,
    Preview
}

async fn fetch_media(media_type: MediaType, url: String) -> Result<Image, Error> {
    let resp = reqwest::get(&url).await.map_err(|_| Error::LabelMe)?;
    if !resp.status().is_success() {
        return Err(Error::LabelMe);
    }

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or_else(|| Error::LabelMe)?
        .to_str()
        .map_err(|_| Error::LabelMe)?;
    
    let mine: mime::Mime = content_type.parse().map_err(|_| Error::LabelMe)?;
    if let mime::IMAGE = mine.type_() {
        let bytes = resp.bytes().await.map_err(|_| Error::LabelMe)?;
        if let Ok(size) = imagesize::blob_size(&bytes) {
            let imagesize::ImageSize { width, height } = size;
            let  width =  width as isize;
            let height = height as isize;

            let media = Media { url, width, height };
            return Ok(
                match media_type {
                    MediaType::Large => Image::Large(media),
                    MediaType::Preview => Image::Preview(media)
                }
            )
        }
    }

    Err(Error::LabelMe)
}
