use crate::command::{self, BotCommand};

use super::{
    message::{Message, MessageEntityType},
    payment::{PreCheckoutQuery, ShippingQuery},
    CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Poll,
    PollAnswer,
};

#[derive(serde::Deserialize)]
pub struct Update {
    pub update_id: u64,
    #[serde(flatten)]
    pub update_type: UpdateType,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
}

impl Update {
    pub fn command<C: BotCommand>(&self) -> Result<Option<C>, command::Error> {
        match &self.update_type {
            UpdateType::Message(message) => Some(message),
            _ => None,
        }
        .as_ref()
        .filter(|m| {
            m.entities
                .as_ref()
                .and_then(|entities| entities.get(0))
                .is_some_and(|en| en.entity_type == MessageEntityType::BotCommand && en.offset == 0)
        })
        .and_then(|m| m.text.as_ref())
        .map(|t| C::parse(t))
        .transpose()
    }
}
