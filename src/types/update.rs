use serde_with::skip_serializing_none;

use super::{
    message::Message,
    payment::{PreCheckoutQuery, ShippingQuery},
    CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Poll,
    PollAnswer,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Update {
    pub update_id: u64,
    #[serde(flatten)]
    pub update_type: UpdateType,
}

#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
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
