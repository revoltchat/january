use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}

#[derive(Debug, Serialize)]
pub enum LightspeedType {
    Channel,
}

#[derive(Debug, Serialize)]
pub enum BandcampType {
    Album,
    Track,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Special {
    None,
    GIF,
    YouTube {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<String>,
    },
    Lightspeed {
        content_type: LightspeedType,
        id: String,
    },
    Twitch {
        content_type: TwitchType,
        id: String,
    },
    Spotify {
        content_type: String,
        id: String,
    },
    Soundcloud,
    Bandcamp {
        content_type: BandcampType,
        id: String,
    },
}
