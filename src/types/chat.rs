use super::message::Message;

/// This object represents a chat.
#[derive(serde::Deserialize)]
pub struct Chat {
    ///
    pub id: u64,
    ///
    pub photo: Option<ChatPhoto>,
    ///
    pub pinned_message: Option<Box<Message>>,
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
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[derive(serde::Deserialize)]
pub struct ChatPermissions {
    #[serde(default)]
    pub can_send_messages: bool,
    #[serde(default)]
    pub can_send_audios: bool,
    #[serde(default)]
    pub can_send_documents: bool,
    #[serde(default)]
    pub can_send_photos: bool,
    #[serde(default)]
    pub can_send_videos: bool,
    #[serde(default)]
    pub can_send_videos_notes: bool,
    #[serde(default)]
    pub can_send_voice_notes: bool,
    #[serde(default)]
    pub can_send_polls: bool,
    #[serde(default)]
    pub can_send_other_messages: bool,
    #[serde(default)]
    pub can_add_web_page_previews: bool,
    #[serde(default)]
    pub can_change_info: bool,
    #[serde(default)]
    pub can_invite_users: bool,
    #[serde(default)]
    pub can_pin_messages: bool,
    #[serde(default)]
    pub can_manage_topics: bool,
}

#[derive(serde::Deserialize)]
pub struct ChatLocation {}
