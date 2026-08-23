use crate::dungeon::Dungeon;
mod dungeon;
mod input;
mod room;
mod room_location;

fn main() {
    let mut dungeon = Dungeon::new();

    loop {
        dungeon.run_input();
    }
}
