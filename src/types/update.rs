use crate::command::{self, BotCommand};

use super::{
    chat::Chat,
    message::{Location, Message, MessageEntityType, Poll},
    payment::{PreCheckoutQuery, ShippingQuery},
    User,
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
    fn command<C: BotCommand>(&self) -> Result<Option<C>, command::Error> {
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

#[derive(serde::Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}

#[derive(serde::Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub optins_ids: Vec<usize>,
}

#[derive(serde::Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: u64,
    pub old_chat_member: ChatMember,
    pub new_chat_mamber: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_chat_folder_invite_link: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(serde::Deserialize)]
pub enum ChatMember {
    Owner {
        status: String,
        user: User,
        is_anonymous: bool,
        custom_title: Option<String>,
    },
    Administrator {
        status: String,
        user: User,
        can_be_edited: bool,
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_pin_messages: Option<bool>,
        can_menage_topics: Option<bool>,
        custom_title: Option<String>,
    },
    Member {
        status: String,
        user: User,
    },
    Restricted {
        status: String,
        user: User,
        is_member: bool,
        can_send_messges: bool,
        can_send_audios: bool,
        can_send_documents: bool,
        can_send_photos: bool,
        can_send_videos: bool,
        can_send_video_notes: bool,
        can_send_polls: bool,
        can_send_other_messages: bool,
        can_add_web_page_previews: bool,
        can_change_info: bool,
        can_invite_users: bool,
        can_pin_messages: bool,
        can_manage_topics: bool,
        until_date: u64,
    },
    Left {
        status: String,
        user: User,
    },
    Banned {
        status: String,
        user: User,
        until_date: u64,
    },
}

#[derive(serde::Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<usize>,
    pub pending_join_reqeust_count: Option<usize>,
}
