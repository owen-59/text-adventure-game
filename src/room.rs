use crate::door_config::DoorConfig;

#[derive(Clone)]
pub struct Room {
    pub door_config: DoorConfig,
}

impl Room {
    pub fn new(door_config: DoorConfig) -> Self {
        Self { door_config }
    }

    pub fn display(&self) -> String {
        let doors_text = if self.door_config.has_doors() {
            format!(
                "There are doors to the {}.",
                self.door_config.doors_list().join(", ")
            )
        } else {
            "".to_string()
        };

        format!("You are in a dark room in a dungeon. {doors_text}")
    }
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("door_config", &self.door_config)
            .finish()
    }
}
