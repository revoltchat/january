use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ImageSize {
    Large,
    Preview,
}

#[derive(Debug, Serialize)]
pub struct Image {
    pub url: String,
    pub width: isize,
    pub height: isize,
    pub size: ImageSize,
}

#[derive(Debug, Serialize)]
pub struct Video {
    pub url: String,
    pub width: isize,
    pub height: isize,
}
