use crate::types::message::MessageEntity;

use super::{ChatId, ReplyMarkup};

#[derive(serde::Serialize, tgbotool_derive::Builder, tgbotool_derive::TgMethod)]
pub struct SendPoll {
    chat_id: ChatId,
    message_thread_id: Option<u64>,
    question: String,
    options: Vec<String>,
    is_anonymous: Option<bool>,
    poll_type: Option<PollType>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<u64>,
    explanation: Option<String>,
    explanation_parse_mode: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<u16>,
    close_date: Option<u64>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<u64>,
    allow_sendign_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Quiz,
    Regular,
}
