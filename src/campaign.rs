
use super::player::Player;

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Campaign {
    pub active: bool,
    pub players: Vec<Player>,
    pub channel_id: String,
    pub creation: Instant 
    
}

impl Campaign {
    pub fn new(channel_id: &str) -> Campaign {
        Campaign { 
            active: false, 
            players: Vec::new(), 
            channel_id: channel_id.to_string(),
            creation: Instant::now() 
        }
    }
}