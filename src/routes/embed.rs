use actix_web::{ web::{self, Query}, Responder };
use serde::{Deserialize};

use crate::{structs::media::{Media, MediaSize}, util::result::Error};
use crate::structs::embed::Embed;
use crate::structs::metadata::Metadata;
use crate::util::request::{consume_metatags, fetch};

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;
    let (resp, mime) = fetch(&url).await?;

    if let mime::HTML = mime.subtype() {
        let properties = consume_metatags(resp).await?;
        let mut metadata = Metadata::from(properties);
        metadata.resolve_external().await;
        
        Ok(web::Json(Embed::Website(metadata)))
    } else if let mime::IMAGE = mime.type_() {
        let bytes = resp.bytes().await.map_err(|_| Error::LabelMe)?;
        if let Ok(size) = imagesize::blob_size(&bytes) {
            let imagesize::ImageSize { width, height } = size;
            let  width =  width as isize;
            let height = height as isize;

            let media = Media { url, width, height, size: MediaSize::Large };
            Ok(web::Json(Embed::Image(media)))
        } else {
            Ok(web::Json(Embed::None))
        }
    } else {
        Ok(web::Json(Embed::None))
    }
}
