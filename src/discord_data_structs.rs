use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub r#type: u8,
    pub data: Option<AppCommand>,
    #[serde(skip)]
    pub guild: String,
    pub guild_id: Option<String>,
    #[serde(skip)]
    pub channel: Option<String>,
    pub channel_id: Option<String>,
    #[serde(skip)]
    pub member: Option<String>,
    pub user: User,   
    pub token: u8,
    pub version: u8,
    #[serde(skip)]
    pub message: Option<String>,
    pub app_permissions: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    #[serde(skip)]
    pub entitlements: Vec<String>,
    #[serde(skip)]
    pub authorizing_integration_owners: Vec<String>, 
    #[serde(skip)]
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
    pub avatar: String,
    #[serde(skip)]
    pub avatar_decoration_data: Option<AvatarDecorationData>,
	pub bot: bool,
    #[serde(skip)]
	pub clan: Option<String>,
    #[serde(skip)]
    pub collectibles: Option<String>,
	pub discriminator: String,
    #[serde(skip)]
	pub display_name_styles: Option<String>,
	pub global_name: String,
	pub id: String,
    #[serde(skip)]
	pub primary_guild: Option<String>,
	pub public_flags: u32,
	pub system: bool,
    #[serde(skip)]
    pub mfa_enabled: Option<bool>,
    #[serde(skip)]
    pub banner: Option<String>,
    pub locale: String,
	pub username: String, 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String
}

