use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub r#type: u8,
    pub guild_id: String,
    pub channel_id: String,
    pub data: Option<AppCommand>   
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

