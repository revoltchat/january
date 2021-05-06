use std::collections::HashMap;

use actix_web::{
    web::{self, Query},
    App, HttpRequest, HttpServer, Responder,
};
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use serde::Deserialize;

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

/*#[derive(Deserialize)]
struct Info {
    url: String,
}

async fn embed(info: Query<Info>) -> impl Responder {
    let url = info.into_inner().url;

    let resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());

    dbg!(resp.headers().get(CONTENT_TYPE));

    let body = resp.text().await.unwrap();
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("meta").unwrap();

    for el in fragment.select(&selector) {
        let node = el.value();
        dbg!(node.attr("property"));
        dbg!(node.attr("content"));
    }

    format!("gaming has been deposited into stdout")
}*/

#[derive(Deserialize)]
struct Parameters {
    url: String,
}

struct Metadata {
    title: Option<String>,
}

async fn embed(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;

    let resp = reqwest::get(url).await.map_err(|_| Error::LabelMe)?;
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

        let mut properties = HashMap::<&str, String>::new();        
        for el in fragment.select(&selector) {
            let node = el.value();

            if let (Some(property), Some(content)) = (node.attr("property"), node.attr("content")) {
                properties.insert(property, content.to_string());
            }
        }

        let mut metadata = Metadata { title: None };
        metadata.title = properties.remove("og:title");

        if let Some(opengraph_type) = properties.remove("og:type") {
            match &opengraph_type[..] {
                "abc" => {},
                _ => {}
            }
        }

        Ok(format!("is html"))
    } else if let mime::IMAGE = mine.type_() {
        let bytes = resp.bytes().await.map_err(|_| Error::LabelMe)?;
        if let Ok(size) = imagesize::blob_size(&bytes) {
            dbg!(size);
            Ok(format!("is image"))
        } else {
            Ok(format!("said it was an image but we couldn't parse it"))
        }
    } else {
        Ok(format!("not html!!"))
    }
}
