use crate::dungeon::Dungeon;
mod door_config;
mod dungeon;
mod enemy;
mod input;
mod input_wrappers;
mod item;
mod player;
mod room;
mod room_location;

fn main() {
    let mut dungeon = Dungeon::new();

    dungeon.display_room();

    loop {
        dungeon.run_input();
    }
}
