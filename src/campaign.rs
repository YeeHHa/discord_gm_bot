
use super::player::Player;

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Campaign {
    pub active: bool,
    pub players: Vec<Player>,
    pub current_player : Option<Player>,
    pub channel_id: String,
    pub creation: Instant 
    
}

impl Campaign {
    pub fn new(channel_id: &str) -> Campaign {
        Campaign { 
            active: false, 
            players: Vec::new(),
            current_player: None, 
            channel_id: channel_id.to_string(),
            creation: Instant::now() 
        }
    }

    pub fn next_player(&mut self) {

        match self.current_player.as_mut() {
            Some(current) => {
                let current_index = self.players.iter().position(|p| p.id == current.id);
                if let Some(index) = current_index {
                    let next_index = (index + 1) % self.players.len();
                    self.current_player = Some(self.players[next_index].clone());
                }
            },
            None => { 
                if !self.players.is_empty() {
                    self.current_player = Some(self.players[0].clone());
                }
            }
            
        }
    }
}