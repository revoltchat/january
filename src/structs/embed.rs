use serde::Serialize;

use super::{media::Media, metadata::Metadata};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Media),
    None
}
