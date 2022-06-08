use serde::Serialize;
use validator::Validate;

#[derive(Debug, Serialize)]
pub enum ImageSize {
    Large,
    Preview,
}

#[derive(Validate, Debug, Serialize)]
pub struct Image {
    #[validate(length(min = 1, max = 512))]
    pub url: String,
    pub width: isize,
    pub height: isize,
    pub size: ImageSize,
}

#[derive(Validate, Debug, Serialize)]
pub struct Video {
    #[validate(length(min = 1, max = 512))]
    pub url: String,
    pub width: isize,
    pub height: isize,
}
