use serde_with::skip_serializing_none;

use crate::types::{
    message::MessageEntity, ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};

pub struct SendMessageBuilder {
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
    reply_markup: Option<ReplyMarkUp>,
}

impl SendMessageBuilder {
    pub fn new(chat_id: ChatId, text: &str) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            text: text.to_owned(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn message_thread_id(mut self, message_thread_id: u64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn parse_mode(mut self, parse_mode: &str) -> Self {
        self.parse_mode = Some(parse_mode.to_owned());
        self
    }

    pub fn add_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.entities.get_or_insert(vec![]);
        entities.push(entity);
        self
    }

    pub fn entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }

    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn reply_to_message_id(mut self, reply_to_message_id: u64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn reply_markup(mut self, reply_markup: ReplyMarkUp) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn build(self) -> SendMessage {
        SendMessage {
            chat_id: self.chat_id,
            message_thread_id: self.message_thread_id,
            text: self.text,
            parse_mode: self.parse_mode,
            entities: self.entities,
            disable_web_page_preview: self.disable_web_page_preview,
            disable_notification: self.disable_notification,
            protect_content: self.protect_content,
            reply_to_message_id: self.reply_to_message_id,
            allow_sending_without_reply: self.allow_sending_without_reply,
            reply_markup: self.reply_markup,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum ChatId {
    Chat(u64),
    Channel(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReplyMarkUp {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[skip_serializing_none]
#[derive(serde::Serialize)]
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
    #[serde(flatten)]
    reply_markup: Option<ReplyMarkUp>,
}
