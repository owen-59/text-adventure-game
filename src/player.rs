use crate::room_location::RoomLocation;

#[derive(Clone)]
pub struct Player {
    pub location: RoomLocation,
    pub health: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            location: RoomLocation { x: 0, y: 0 },
            health: 120,
        }
    }
}
