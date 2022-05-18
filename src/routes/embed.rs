use actix_web::{
    web::{self, Query},
    Responder,
};
use serde::Deserialize;

use crate::structs::metadata::Metadata;
use crate::structs::{embed::Embed, media::Video};
use crate::util::request::fetch;
use crate::{
    structs::media::{Image, ImageSize},
    util::{request::consume_size, result::Error},
};

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;
    let (resp, mime) = fetch(&url).await?;

    match (mime.type_(), mime.subtype()) {
        (_, mime::HTML) => {
            let mut metadata = Metadata::from(resp, url).await?;
            metadata.resolve_external().await;

            if metadata.is_none() {
                return Ok(web::Json(Embed::None));
            }

            Ok(web::Json(Embed::Website(metadata)))
        }
        (mime::IMAGE, _) => {
            if let Ok((width, height)) = consume_size(resp, mime).await {
                Ok(web::Json(Embed::Image(Image {
                    url,
                    width,
                    height,
                    size: ImageSize::Large,
                })))
            } else {
                Ok(web::Json(Embed::None))
            }
        }
        (mime::VIDEO, _) => {
            if let Ok((width, height)) = consume_size(resp, mime).await {
                Ok(web::Json(Embed::Video(Video { url, width, height })))
            } else {
                Ok(web::Json(Embed::None))
            }
        }
        _ => Ok(web::Json(Embed::None)),
    }
}
