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
    pub guild: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<AuthorizingIntegrationOwners>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    pub attachment_size_limit: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AppCommand {
    pub name: String,
    pub command_type: u8,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseOject {
    pub r#type: u8,
    pub data: Option<MessageObject>
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
    #[serde(skip_serializing_if = "Option::is_none")]
	pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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