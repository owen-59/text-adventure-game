#[derive(Hash, Eq, PartialEq, Clone)]
pub struct RoomLocation {
    pub x: i32,
    pub y: i32,
}

impl RoomLocation {
    pub fn copy_with_diff(&self, x: i32, y: i32) -> Self {
        RoomLocation {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
