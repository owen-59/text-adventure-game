use crate::room_location::RoomLocation;

pub struct Player {
    pub location: RoomLocation,
}

impl Player {
    pub fn new() -> Self {
        Self {
            location: RoomLocation { x: 0, y: 0 },
        }
    }
}
