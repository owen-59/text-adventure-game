#[derive(Clone)]
pub struct DoorConfig {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl DoorConfig {
    pub fn has_doors(&self) -> bool {
        self.north || self.east || self.south || self.west
    }

    pub fn doors_list(&self) -> Vec<&str> {
        [
            if self.north { Some("north") } else { None },
            if self.east { Some("east") } else { None },
            if self.south { Some("south") } else { None },
            if self.west { Some("west") } else { None },
        ]
        .iter()
        .flatten()
        .cloned()
        .collect()
    }
}

impl std::fmt::Debug for DoorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoorConfig")
            .field("north", &self.north)
            .field("east", &self.east)
            .field("south", &self.south)
            .field("west", &self.west)
            .finish()
    }
}
