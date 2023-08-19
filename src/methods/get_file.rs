use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(serde::Serialize, tgbotool_derive::TgMethod)]
pub struct GetFile {
    file_id: String,
}

impl GetFile {
    pub fn new(file_id: &str) -> Self {
        Self {
            file_id: file_id.to_owned(),
        }
    }
}
