use serde_with::skip_serializing_none;

use crate::types::message::MessageEntity;

use super::{ChatId, ReplyMarkup, SendFile};

macro_rules! impl_thumbnail {
    ($builder: path) => {
        impl $builder {
            pub fn thumbnail(mut self, file: super::UploadFile) -> Self {
                self.thumbnail = Some(SendFile::UploadInput(file));
                self
            }
        }
    };
}

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendPhoto {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    photo: SendFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl SendPhoto {
    pub fn is_multipart(&self) -> bool {
        matches!(self.photo, SendFile::UploadInput { .. })
    }
}

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendAudio {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    audio: SendFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<usize>,
    performer: Option<String>,
    title: Option<String>,
    // force to upload local file
    #[builder(skip)]
    #[multipart(attach)]
    thumbnail: Option<SendFile>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl_thumbnail!(SendAudioBuilder);

impl SendAudio {
    pub(crate) fn is_multipart(&self) -> bool {
        matches!(self.audio, SendFile::UploadInput { .. })
            || matches!(self.thumbnail, Some(SendFile::UploadInput { .. }))
    }
}

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendDocument {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    document: SendFile,
    #[builder(skip)]
    #[multipart(attach)]
    thumbnail: Option<SendFile>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}
impl_thumbnail!(SendDocumentBuilder);

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendVideo {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    video: SendFile,
    duration: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
    #[builder(skip)]
    #[multipart(attach)]
    thumbnail: Option<SendFile>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    supports_streaming: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl_thumbnail!(SendVideoBuilder);

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendAnimation {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    animation: SendFile,
    duration: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
    #[builder(skip)]
    #[multipart(attach)]
    thumbnail: Option<SendFile>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_spoiler: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl_thumbnail!(SendAnimationBuilder);

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendVoice {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    voice: SendFile,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<usize>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

#[skip_serializing_none]
#[derive(
    serde::Serialize,
    tgbotool_derive::TgMethod,
    tgbotool_derive::Builder,
    tgbotool_derive::Multipart,
)]
pub struct SendVideoNote {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    #[multipart(normal)]
    video_note: SendFile,
    duration: Option<usize>,
    length: Option<usize>,
    #[builder(skip)]
    #[multipart(attach)]
    thumbnail: Option<SendFile>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl_thumbnail!(SendVideoNote);
