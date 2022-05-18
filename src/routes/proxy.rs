use actix_web::{web::Query, HttpResponse, Responder};
use serde::Deserialize;

use crate::util::request::fetch;
use crate::util::result::Error;

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;
    let (resp, mime) = fetch(&url).await?;

    if matches!(mime.type_(), mime::IMAGE | mime::VIDEO) {
        let body = resp
            .bytes()
            .await
            .map_err(|_| Error::FailedToConsumeBytes)?;
        Ok(HttpResponse::Ok().body(body))
    } else {
        Err(Error::NotAllowedToProxy)
    }
}
