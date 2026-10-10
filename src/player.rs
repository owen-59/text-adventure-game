use crate::{
    item::{Item, Weapon},
    room_location::RoomLocation,
};

#[derive(Clone)]
pub struct Player {
    pub location: RoomLocation,
    pub health: i32,
    pub items: Vec<Item>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            location: RoomLocation { x: 0, y: 0 },
            health: 120,
            items: vec![Box::new(Weapon::new_fist())],
        }
    }

    pub fn display_items(&self) -> String {
        if self.items.is_empty() || self.items.len() == 1 {
            "You have absolutely nothing.".to_string()
        } else {
            format!(
                "You have a {}.",
                self.items
                    .iter()
                    .filter(|item| item.should_show())
                    .map(|item| item.display())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}
