use super::message::Message;

#[derive(serde::Deserialize)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: ChosenInlineResult,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>,
    pub chat_join_request: Option<ChatJoinRequest>,
}

#[derive(serde::Deserialize)]
pub struct InlineQuery {}

#[derive(serde::Deserialize)]
pub struct ChosenInlineResult {}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {}

#[derive(serde::Deserialize)]
pub struct ShippingQuery {}

#[derive(serde::Deserialize)]
pub struct PreCheckoutQuery {}

#[derive(serde::Deserialize)]
pub struct Poll {}

#[derive(serde::Deserialize)]
pub struct PollAnswer {}

#[derive(serde::Deserialize)]
pub struct ChatMemberUpdated {}

#[derive(serde::Deserialize)]
pub struct ChatJoinRequest {}
