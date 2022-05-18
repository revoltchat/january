use serde::Serialize;

use super::{media::Image, metadata::Metadata};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    None,
}
