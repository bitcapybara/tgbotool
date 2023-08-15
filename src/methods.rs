use std::path::PathBuf;

use crate::types::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};

pub mod answer_callback_query;
pub mod send_message;
pub mod send_photo;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum ChatId {
    Chat(u64),
    Channel(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl ReplyMarkup {
    pub fn inline_keyboard(keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self::InlineKeyboardMarkup(InlineKeyboardMarkup::new(keyboard))
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum File {
    Input(PathBuf),
    FileId(String),
}
