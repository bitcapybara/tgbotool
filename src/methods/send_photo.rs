use serde_with::skip_serializing_none;

use crate::types::message::MessageEntity;

use super::{ChatId, ReplyMarkup, SendFile};

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder, tgbotool_derive::Multipart)]
#[multipart(field = "photo")]
pub struct SendPhoto {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
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
    pub(crate) fn is_multipart(&self) -> bool {
        matches!(self.photo, SendFile::UploadInput { .. })
    }
}
