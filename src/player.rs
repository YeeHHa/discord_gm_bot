

#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub display_name: String,
    pub health: u8,
}

impl Player {
    pub fn new(id: String, display_name:String) -> Player {
        Player {
            id: id,
            display_name: display_name,
            health: 10
        }
    }
}