use actix_web::{web::Query, HttpResponse, Responder};
use serde::Deserialize;

use crate::util::request::{fetch, get_bytes};
use crate::util::result::Error;

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;
    let (mut resp, mime) = fetch(&url).await?;

    if matches!(mime.type_(), mime::IMAGE | mime::VIDEO) {
        let bytes = get_bytes(&mut resp).await?;
        Ok(HttpResponse::Ok().body(bytes))
    } else {
        Err(Error::NotAllowedToProxy)
    }
}
