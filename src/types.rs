use serde_with::skip_serializing_none;

use self::{
    chat::Chat,
    message::{Message, MessageEntity},
    user::User,
};

pub mod chat;
pub mod game;
pub mod message;
pub mod passport;
pub mod payment;
pub mod update;
pub mod user;

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

#[derive(serde::Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub duration: u64,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: usize,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
    pub thumbnail: Option<PhotoSize>,
}

#[derive(serde::Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub width: usize,
    pub height: usize,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

#[derive(serde::Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: usize,
    pub duration: usize,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: usize,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<u64>,
    pub vcard: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: usize,
}

#[derive(serde::Deserialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: usize,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub poll_type: String,
    pub allows_multiple_anwsers: bool,
    pub correct_option_id: Option<u64>,
    pub explanation: Option<String>,
    pub explanation_entities: Vec<MessageEntity>,
    pub open_period: Option<usize>,
    pub close_date: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct PollOption {
    pub text: Option<String>,
    pub voter_count: usize,
}

#[derive(serde::Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<usize>,
    pub heading: Option<usize>,
    pub proximity_alert_radius: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: usize,
}

#[derive(serde::Deserialize)]
pub struct UserShared {
    pub request_id: u64,
    pub user_id: u64,
}

#[derive(serde::Deserialize)]
pub struct ChatShared {
    pub request_id: u64,
    pub chat_id: u64,
}

#[derive(serde::Deserialize)]
pub struct WriteAccessAllowed {
    pub web_app_name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: usize,
}

#[derive(serde::Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: usize,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ForumTopicClosed {}

#[derive(serde::Deserialize)]
pub struct ForumTopicReopened {}

#[derive(serde::Deserialize)]
pub struct GeneralForumTopicHidden {}

#[derive(serde::Deserialize)]
pub struct GeneralForumTopicUnHidden {}

#[derive(serde::Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: u64,
}

#[derive(serde::Deserialize)]
pub struct VideoChatStarted {}

#[derive(serde::Deserialize)]
pub struct VideoChatEnded {
    pub duration: u64,
}

#[derive(serde::Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

#[derive(serde::Deserialize)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { inline_keyboard }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardMarkup {}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardRemove {}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForceReply {}

#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, tgbotool_derive::Builder)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackGame {}
