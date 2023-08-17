use reqwest::multipart;
use serde_with::skip_serializing_none;

use crate::types::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};

pub mod answer_callback_query;
pub mod get_file;
pub mod get_updates;
pub mod send_media_group;
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

#[skip_serializing_none]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum SendFile {
    UploadInput {
        file_name: String,
        file_bytes: Vec<u8>,
    },
    FileIdOrUrl(String),
}

impl SendFile {
    pub fn file_id_or_url<S>(id: S) -> Self
    where
        S: Into<String>,
    {
        Self::FileIdOrUrl(id.into())
    }

    pub fn input(file_name: &str, input: Vec<u8>) -> Self {
        Self::UploadInput {
            file_name: file_name.to_string(),
            file_bytes: input,
        }
    }
    pub fn into_part(self) -> reqwest::multipart::Part {
        match self {
            SendFile::UploadInput {
                file_name,
                file_bytes,
            } => multipart::Part::bytes(file_bytes).file_name(file_name),
            SendFile::FileIdOrUrl(s) => reqwest::multipart::Part::text(s),
        }
    }
}
