use std::collections::HashMap;

use mime::Mime;
use reqwest::{header::CONTENT_TYPE, Response};
use scraper::{Html, Selector};

use super::result::Error;

pub async fn fetch(url: &str) -> Result<(Response, Mime), Error> {
    let resp = reqwest::get(url).await.map_err(|_| Error::ReqwestFailed)?;
    if !resp.status().is_success() {
        dbg!(resp.text().await.unwrap());
        return Err(Error::RequestFailed);
    }

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or_else(|| Error::MissingContentType)?
        .to_str()
        .map_err(|_| Error::ConversionFailed)?;
    
    let mime: mime::Mime = content_type.parse().map_err(|_| Error::FailedToParseContentType)?;
    Ok((resp, mime))
}

pub async fn consume_metatags(resp: Response) -> Result<HashMap<String, String>, Error> {
    let body = resp.text().await.map_err(|_| Error::FailedToConsumeText)?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("meta").map_err(|_| Error::MetaSelectionFailed)?;

    let mut properties = HashMap::new();
    for el in fragment.select(&selector) {
        let node = el.value();

        if let (Some(property), Some(content)) = (node.attr("property"), node.attr("content")) {
            properties.insert(property.to_string(), content.to_string());
        }
    }

    Ok(properties)
}

pub async fn consume_size(resp: Response) -> Result<(isize, isize), Error> {
    let bytes = resp.bytes().await.map_err(|_| Error::FailedToConsumeBytes)?;
    if let Ok(size) = imagesize::blob_size(&bytes) {
        Ok((size.width as isize, size.height as isize))
    } else {
        Err(Error::CouldNotDetermineImageSize)
    }
}
