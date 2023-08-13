use super::{
    chat::Chat,
    game::Game,
    passport::PassportData,
    payment::{Invoice, SuccessfulPayment},
    User,
};

#[derive(serde::Deserialize)]
pub struct Message {
    pub message_id: u64,
    pub message_thread_id: Option<u64>,
    pub from: Option<User>,
    pub sender_chat: Option<Box<Chat>>,
    pub date: u64,
    pub chat: Box<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<u64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<u64>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<PhotoSize>,
    #[serde(default)]
    pub delete_chat_photo: bool,
    #[serde(default)]
    pub group_chat_created: bool,
    #[serde(default)]
    pub supergroup_chat_created: bool,
    #[serde(default)]
    pub channel_chat_created: bool,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<u64>,
    pub migrate_from_chat_id: Option<u64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub user_shared: Option<UserShared>,
    pub chat_shared: Option<ChatShared>,
    pub connected_website: Option<String>,
    pub write_access_allowd: Option<WriteAccessAllowed>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_edited: Option<ForumTopicEdited>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    pub genenral_forum_topic_unhidden: Option<GeneralForumTopicUnHidden>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(serde::Deserialize)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub entity_type: MessageEntityType,
    pub offset: usize,
    pub length: usize,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
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

#[derive(serde::Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(serde::Deserialize)]
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

#[derive(serde::Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct CallbackGame {}
