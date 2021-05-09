use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Special {
    None,
    YouTube {
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
}
