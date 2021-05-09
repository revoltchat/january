use serde::Serialize;

use super::{media::Image, metadata::Metadata};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    None,
}
