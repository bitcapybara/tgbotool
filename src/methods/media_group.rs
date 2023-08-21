use std::mem;

use serde_with::skip_serializing_none;

use crate::types::message::MessageEntity;

use super::{ChatId, SendFile, TgMultipartMethod, UploadFile};

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct SendMediaGroup {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    media: Vec<Media>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
}

impl TgMultipartMethod for SendMediaGroup {
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

impl TryFrom<SendMediaGroup> for reqwest::multipart::Form {
    type Error = serde_json::Error;

    fn try_from(mut this: SendMediaGroup) -> Result<Self, Self::Error> {
        use serde_json as json;
        let mut form = reqwest::multipart::Form::new();

        form = form.text("chat_id", json::to_string(&this.chat_id)?);

        if let Some(message_thread_id) = this.message_thread_id {
            form = form.text("message_thread_id", json::to_string(&message_thread_id)?);
        }

        for media in &mut this.media {
            if let Some((file_name, part)) = media.get_file_part() {
                form = form.part(file_name, part);
            }
        }
        form = form.text("media", json::to_string(&this.media)?);

        if let Some(disable_notification) = this.disable_notification {
            form = form.text(
                "disable_notification",
                json::to_string(&disable_notification)?,
            );
        }

        if let Some(protect_content) = this.protect_content {
            form = form.text("protect_content", json::to_string(&protect_content)?);
        }

        if let Some(reply_to_message_id) = this.reply_to_message_id {
            form = form.text(
                "reply_to_message_id",
                json::to_string(&reply_to_message_id)?,
            );
        }

        if let Some(allow_sending_without_reply) = this.allow_sending_without_reply {
            form = form.text(
                "allow_sending_without_reply",
                json::to_string(&allow_sending_without_reply)?,
            );
        }

        Ok(form)
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

    fn get_file_part(&mut self) -> Option<(String, reqwest::multipart::Part)> {
        use reqwest::multipart::Part;
        let media = match self {
            Media::Audio(m) => &mut m.media,
            Media::Document(m) => &mut m.media,
            Media::Photo(m) => &mut m.media,
            Media::Video(m) => &mut m.media,
        };
        if let SendFile::UploadInput(UploadFile {
            file_name,
            file_bytes,
        }) = media
        {
            let attach_name = file_name.clone();
            let part = Part::bytes(mem::take(file_bytes)).file_name(mem::take(file_name));
            *media = SendFile::FileIdOrUrl(format!("attach://{attach_name}"));
            return Some((attach_name.clone(), part));
        }
        None
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
