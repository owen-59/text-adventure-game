use crate::{
    door_config::DoorConfig,
    enemy::{Enemy, ShouldDelete},
    item::Item,
    player::Player,
};

#[derive(Clone)]
pub struct Room {
    pub door_config: DoorConfig,
    pub enemy: Option<Enemy>,
    pub items: Vec<Item>,
}

impl Room {
    pub fn new(door_config: DoorConfig, enemy: Option<Enemy>, items: Vec<Item>) -> Self {
        Self {
            door_config,
            enemy,
            items,
        }
    }

    pub fn display(&mut self) -> String {
        let doors_text = if self.door_config.has_doors() {
            format!(
                "There are doors to the {}.",
                self.door_config.doors_list().join(", ")
            )
        } else {
            "".to_string()
        };

        let enemy_text = if let Some(enemy) = &mut self.enemy {
            enemy.display()
        } else {
            "".to_string()
        };

        let items_text = if self.items.is_empty() {
            "".to_string()
        } else {
            format!(
                "There is {} on the ground.",
                self.items
                    .iter()
                    .map(|item| item.display())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        format!(
            "You are in a dark room in a dungeon. {} {} {}",
            doors_text, enemy_text, items_text
        )
    }

    pub fn cycle(&mut self, player: &mut Player) {
        if let Some(enemy) = &mut self.enemy {
            match enemy.cycle(player) {
                ShouldDelete::Delete => self.enemy = None,
                ShouldDelete::DoNotDelete => {}
            };
        }
    }
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("door_config", &self.door_config)
            .finish()
    }
}
