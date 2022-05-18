use mime::Mime;
use reqwest::{header::CONTENT_TYPE, Client, Response};
use scraper::Html;

use super::result::Error;

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (compatible; January/1.0; +https://gitlab.insrt.uk/revolt/january)"
        )
        .build()
        .unwrap();
}

pub async fn fetch(url: &str) -> Result<(Response, Mime), Error> {
    let resp = CLIENT
        .get(url)
        .send()
        .await
        .map_err(|_| Error::ReqwestFailed)?;

    if !resp.status().is_success() {
        return Err(Error::RequestFailed);
    }

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE).ok_or(Error::MissingContentType)?
        .to_str()
        .map_err(|_| Error::ConversionFailed)?;

    let mime: mime::Mime = content_type
        .parse()
        .map_err(|_| Error::FailedToParseContentType)?;
    
    Ok((resp, mime))
}

pub async fn consume_fragment(resp: Response) -> Result<Html, Error> {
    let body = resp.text().await.map_err(|_| Error::FailedToConsumeText)?;
    Ok(Html::parse_document(&body))
}

pub async fn consume_size(resp: Response) -> Result<(isize, isize), Error> {
    let bytes = resp
        .bytes()
        .await
        .map_err(|_| Error::FailedToConsumeBytes)?;
    if let Ok(size) = imagesize::blob_size(&bytes) {
        Ok((size.width as isize, size.height as isize))
    } else {
        Err(Error::CouldNotDetermineImageSize)
    }
}
