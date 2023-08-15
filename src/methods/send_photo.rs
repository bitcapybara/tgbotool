use crate::types::message::MessageEntity;

use super::{ChatId, File, ReplyMarkup};

#[derive(serde::Serialize, tgbotool_derive::Builder)]
pub struct SendPhoto {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    photo: File,
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
