use reqwest::multipart;

use crate::types::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};

pub mod answer_callback_query;
pub mod get_file;
pub mod get_updates;
pub mod send_media;
pub mod send_media_group;
pub mod send_message;
pub mod send_poll;

pub trait TgMethod: serde::Serialize {
    fn method_name() -> String;
}

pub trait TgMultipartMethod:
    serde::Serialize + TryInto<reqwest::multipart::Form, Error = serde_json::Error>
{
    fn method_name() -> String;

    fn is_multipart(&self) -> bool {
        false
    }
}

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
pub enum SendFile {
    UploadInput(UploadFile),
    FileIdOrUrl(String),
}

#[derive(serde::Serialize)]
pub struct UploadFile {
    file_name: String,
    file_bytes: Vec<u8>,
}

impl UploadFile {
    pub fn new(file_name: &str, file_bytes: Vec<u8>) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_bytes,
        }
    }
}

pub enum FilePart {
    Simple(reqwest::multipart::Part),
    Complex((String, reqwest::multipart::Part)),
}

impl SendFile {
    pub fn id_or_url<S>(id: S) -> Self
    where
        S: Into<String>,
    {
        Self::FileIdOrUrl(id.into())
    }

    pub fn upload(file_name: &str, input: Vec<u8>) -> Self {
        Self::UploadInput(UploadFile {
            file_name: file_name.to_string(),
            file_bytes: input,
        })
    }
}

/// used by normal multipart
impl From<SendFile> for reqwest::multipart::Part {
    fn from(this: SendFile) -> Self {
        match this {
            SendFile::UploadInput(UploadFile {
                file_name,
                file_bytes,
            }) => multipart::Part::bytes(file_bytes).file_name(file_name),
            SendFile::FileIdOrUrl(s) => reqwest::multipart::Part::text(s),
        }
    }
}

/// used by attach multipart
impl From<SendFile> for Vec<FilePart> {
    fn from(this: SendFile) -> Self {
        let mut res = Vec::new();
        match this {
            SendFile::UploadInput(UploadFile {
                file_name,
                file_bytes,
            }) => {
                res.push(FilePart::Simple(multipart::Part::text(format!(
                    "attach://{file_name}"
                ))));
                res.push(FilePart::Complex((
                    file_name.to_owned(),
                    multipart::Part::bytes(file_bytes).file_name(file_name.clone()),
                )))
            }
            SendFile::FileIdOrUrl(s) => res.push(FilePart::Simple(multipart::Part::text(s))),
        }
        res
    }
}
