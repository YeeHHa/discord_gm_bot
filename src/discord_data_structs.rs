use std::hash::Hash;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum AuthorizingIntegrationOwner {
    GuildInstall(Option<u64>),
    UserInstall(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: String,

    pub application_id: String,

    pub r#type: u8,

    pub data: Option<AppCommand>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,   

    pub token: String,

    pub version: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    pub app_permissions: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,

    pub entitlements: Option<Vec<Entitlement>>,

    pub authorizing_integration_owners: Option<HashMap<String, String>>, 

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,

    pub attachment_size_limit: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AppCommand {
    pub name: String,
    pub r#type: u8,
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseOject {
    pub r#type: u8,
    pub data: Option<MessageObject>
}

impl ResponseOject {
    pub fn new(message: String) -> ResponseOject {

        let message = MessageObject{
            content: message
        };

        ResponseOject {
            r#type: 4,
            data: Some(message)
        } 
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Pong {
    pub r#type: u8,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MessageObject{
    pub content: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Command {
    pub name: String,
    pub r#type: u8,
    pub description: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Commands {
    pub commands: Vec<Command>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
	pub id: String,
	
    pub username: String, 
	
    pub discriminator: String,
	
    pub global_name: String,
   
    pub avatar: String,
  
	pub bot: Option<bool>,
 
	pub system: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u32>,

    pub public_flags: Option<u32>,

    pub avatar_decoration_data: Option<AvatarDecorationData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<PrimaryGuild>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PrimaryGuild {
    pub identity_guild_id: Option<String>,
    pub identity_enabled: Option<bool>,
    pub tag: String,
    pub badge: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorizingIntegrationOwners(pub Vec<AuthorizingIntegrationOwner>);


#[derive(Deserialize, Serialize, Debug)]
pub struct Entitlement {
    pub id: String,
    pub sku_id: Option<String>,
    pub application_id: String,
    pub user_id: Option<String>,
    pub r#type: u32,
    pub deleted: bool,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub guild_id: Option<String>,
    pub consumed: bool
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
    pub id: String,
    pub r#type: u8,
    pub guild_id: Option<String>,
    pub position: Option<i32>,

    #[serde(skip)]
    pub permission_overwrites: Option<String>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<User>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<i32>,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    #[serde(skip)]
    pub thread_metadata: Option<String>,
    #[serde(skip)]
    pub member: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    #[serde(skip)]
    pub available_tags: Option<String>,
    pub applied_tags: Option<Vec<String>>,
    #[serde(skip)]
    pub default_reaction_emoji: Option<String>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<i32>,
    pub default_forum_layout: Option<i32>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Guild {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Option<String>,
    pub permissions: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    #[serde(skip)]
    pub roles: Option<String>,
    #[serde(skip)]
    pub emojis: Option<String>,
    #[serde(skip)]
    pub features: Option<String>,
    pub mfa_level: Option<u8>,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: Option<u32>,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u32>,
    pub max_members: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: Option<u8>,
    pub premium_subscription_count: Option<u32>,
    pub preferred_locale: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u32>,
    pub max_stage_video_channel_users: Option<u32>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub welcome_screen: Option<String>,
    pub nsfw_level: Option<u8>,
    #[serde(skip)]
    pub stickers: Option<String>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub safety_alerts_channel_id: Option<String>,
    #[serde(skip)]
    pub incidents_data: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub roles: Option<Vec<String>>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}