use actix_web::{HttpResponse, Responder, web::Query};
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

    if let mime::IMAGE = mime.type_() {
        let body = resp.bytes().await.map_err(|_| Error::FailedToConsumeBytes)?;
        Ok(
            HttpResponse::Ok()
                .body(body)
        )
    } else {
        Err(Error::NotAllowedToProxy)
    }
}
