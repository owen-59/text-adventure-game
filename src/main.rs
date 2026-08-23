use crate::dungeon::Dungeon;
mod cardinal_direction;
mod door_config;
mod dungeon;
mod input;
mod player;
mod room;
mod room_location;

fn main() {
    let mut dungeon = Dungeon::new();

    let location = dungeon.player.location.clone();
    let room = dungeon.get_room(&location);

    println!("{}", room.display());

    loop {
        dungeon.run_input();
    }
}
