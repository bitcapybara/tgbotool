use super::{message::MessageEntity, Animation, PhotoSize};

#[derive(serde::Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entites: Vec<MessageEntity>,
    pub animation: Option<Animation>,
}
