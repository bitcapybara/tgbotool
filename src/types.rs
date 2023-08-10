/// This object represents a Telegram user or bot
#[derive(serde::Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: u64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    pub language_code: Option<String>,
    /// Optional. True, if this user is a Telegram Premium user
    #[serde(default)]
    pub is_premium: bool,
    /// Optional. True, if this user added the bot to the attachment menu
    #[serde(default)]
    pub added_to_attachment_menu: bool,
    /// Optional. True, if the bot can be invited to groups. Returned only in `getMe`.
    pub can_join_groups: Option<bool>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in `getMe`.
    pub can_read_all_group_messages: Option<bool>,
    /// Optional. True, if the bot supports inline queries. Returned only in `getMe`.
    pub supports_inline_queries: Option<bool>,
}

/// This object represents a chat.
#[derive(serde::Deserialize)]
pub struct Chat {
    ///
    pub id: u64,
    ///
    pub photo: Option<ChatPhoto>,
    ///
    pub pinned_message: Option<Message>,
    ///
    pub message_auto_delete_time: Option<u64>,
    ///
    pub has_hidden_members: Option<bool>,
    ///
    pub has_protected_content: Option<bool>,
    ///
    #[serde(flatten)]
    #[serde(rename = "type")]
    pub chat_type: ChatType,
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private {
        username: Option<String>,
        first_name: String,
        last_name: Option<String>,
        active_usernames: Option<Vec<String>>,
        emoji_status_custom_emoji_id: Option<String>,
        bio: Option<String>,
        #[serde(default)]
        has_private_forwards: bool,
        #[serde(default)]
        has_restricted_voice_and_video_messages: bool,
    },
    Group {
        title: String,
        description: Option<String>,
        invite_link: Option<String>,
        permissions: Option<ChatPermissions>,
    },
    Supergroup {
        title: String,
        username: Option<String>,
        #[serde(default)]
        is_forum: bool,
        active_usernames: Option<Vec<String>>,
        #[serde(default)]
        join_to_send_messages: bool,
        #[serde(default)]
        join_by_request: bool,
        description: Option<String>,
        invite_link: Option<String>,
        permissions: Option<ChatPermissions>,
        slow_mode_delay: Option<u64>,
        has_aggressive_anti_spam_enabled: Option<bool>,
        sticker_set_name: Option<String>,
        can_set_sticker_set: Option<bool>,
        linked_chat_id: Option<u64>,
        location: Option<ChatLocation>,
    },
    Channel {
        title: String,
        username: Option<String>,
        active_usernames: Option<Vec<String>>,
        description: Option<String>,
        invite_link: Option<String>,
        linked_chat_id: Option<u64>,
    },
}

#[derive(serde::Deserialize)]
pub struct ChatPhoto {}

#[derive(serde::Deserialize)]
pub struct ChatPermissions {}

#[derive(serde::Deserialize)]
pub struct Message {}

#[derive(serde::Deserialize)]
pub struct ChatLocation {}
