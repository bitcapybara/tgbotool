use crate::command::{self, BotCommand};

use super::{
    message::{Message, MessageEntityType},
    payment::ShippingQuery,
    User,
};

#[derive(serde::Deserialize)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>,
    pub chat_join_request: Option<ChatJoinRequest>,
}

impl Update {
    fn command<C: BotCommand>(&self) -> Result<Option<C>, command::Error> {
        self.message
            .as_ref()
            .filter(|m| {
                m.entities
                    .as_ref()
                    .and_then(|entities| entities.get(0))
                    .is_some_and(|en| {
                        en.entity_type == MessageEntityType::BotCommand && en.offset == 0
                    })
            })
            .and_then(|m| m.text.as_ref())
            .map(|t| C::parse(t))
            .transpose()
    }
}

#[derive(serde::Deserialize)]
pub struct InlineQuery {}

#[derive(serde::Deserialize)]
pub struct ChosenInlineResult {}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {}

#[derive(serde::Deserialize)]
pub struct PreCheckoutQuery {}

#[derive(serde::Deserialize)]
pub struct Poll {}

#[derive(serde::Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub optins_ids: Vec<usize>,
}

#[derive(serde::Deserialize)]
pub struct ChatMemberUpdated {}

#[derive(serde::Deserialize)]
pub struct ChatJoinRequest {}
