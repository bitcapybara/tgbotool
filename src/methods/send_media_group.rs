use serde_with::skip_serializing_none;

use crate::types::message::MessageEntity;

use super::{ChatId, FilePart, SendFile, TgMethod};

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder, tgbotool_derive::Multipart)]
pub struct SendMediaGroup {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(attach)]
    media: Vec<Media>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
}

impl TgMethod for SendMediaGroup {
    fn method_name() -> String {
        "sendMediaGroup".to_string()
    }

    fn is_multipart(&self) -> bool {
        for media in &self.media {
            if media.is_multipart() {
                return true;
            }
        }
        false
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum Media {
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl Media {
    fn get_media_mut(&mut self) -> &mut SendFile {
        match self {
            Media::Audio(m) => &mut m.media,
            Media::Document(m) => &mut m.media,
            Media::Photo(m) => &mut m.media,
            Media::Video(m) => &mut m.media,
        }
    }

    fn get_media(&self) -> &SendFile {
        match self {
            Media::Audio(m) => &m.media,
            Media::Document(m) => &m.media,
            Media::Photo(m) => &m.media,
            Media::Video(m) => &m.media,
        }
    }

    fn is_multipart(&self) -> bool {
        matches!(self.get_media(), SendFile::UploadInput { .. })
    }
}

impl From<Media> for Vec<FilePart> {
    fn from(this: Media) -> Self {
        match this {
            Media::Audio(m) => m.media.into(),
            Media::Document(m) => m.media.into(),
            Media::Photo(m) => m.media.into(),
            Media::Video(m) => m.media.into(),
        }
    }
}

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct InputMediaAudio {
    #[serde(rename = "type")]
    #[builder(value = "audio")]
    media_type: String,
    /// file_id / http_url / attach
    media: SendFile,
    thumbnail: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<u64>,
    performer: Option<String>,
    title: Option<String>,
}

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct InputMediaDocument {
    #[serde(rename = "type")]
    #[builder(value = "document")]
    media_type: String,
    /// file_id / http_url / attach
    media: SendFile,
    thumbnail: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
}

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct InputMediaPhoto {
    #[serde(rename = "type")]
    #[builder(value = "photo")]
    media_type: String,
    /// file_id / http_url / attach
    media: SendFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
}

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct InputMediaVideo {
    #[serde(rename = "type")]
    #[builder(value = "video")]
    media_type: String,
    /// file_id / http_url / attach
    media: SendFile,
    thumbnail: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    width: Option<usize>,
    height: Option<usize>,
    duration: Option<usize>,
    supports_streaming: Option<bool>,
    has_spoiler: Option<bool>,
}
