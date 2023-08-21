use serde_with::skip_serializing_none;

use crate::types::message::MessageEntity;

use super::{ChatId, ReplyMarkup};

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::Builder, tgbotool_derive::TgMethod)]
pub struct SendMessage {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}
