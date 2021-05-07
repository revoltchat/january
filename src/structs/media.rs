use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MediaSize {
    Large,
    Preview,
}

#[derive(Debug, Serialize)]
pub struct Media {
    pub url: String,
    pub width: isize,
    pub height: isize,
    pub size: MediaSize,
}
