use std::{process::exit, thread, time};

use rand::random_bool;

use crate::{
    input_wrappers::{AttackInput, get_attack_input},
    player::Player,
};

#[derive(Clone, strum::Display)]
pub enum EnemyType {
    #[strum(to_string = "bat")]
    Bat,
}

pub enum ShouldDelete {
    Delete,
    DoNotDelete,
}

#[derive(Clone)]
pub struct Enemy {
    has_been_seen: bool,
    attack_chance: f64,
    enemy_type: EnemyType,
    health: i32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, attack_chance: f64) -> Self {
        Self {
            has_been_seen: false,
            enemy_type,
            attack_chance,
            health: match enemy_type {
                EnemyType::Bat => 50,
            },
        }
    }

    pub fn display(&mut self) -> String {
        if !self.has_been_seen {
            self.has_been_seen = true;
            format!("There is an enemy {} in this room. ", self.enemy_type).to_string()
        } else {
            format!("{} already seen. ", self.enemy_type).to_string()
        }
    }

    pub fn cycle(&mut self, player: &mut Player) -> ShouldDelete {
        let will_attack = random_bool(self.attack_chance / 256.0);

        if will_attack {
            self.attack_cycle(player);
            ShouldDelete::Delete
        } else {
            println!("The {} has left you alone for now.", self.enemy_type);
            ShouldDelete::DoNotDelete
        }
    }

    fn do_damage_to_enemy(&mut self, damage: i32, miss_chance: f64) {
        let did_miss = random_bool(miss_chance);
        if did_miss {
            println!("You missed.");
        } else {
            self.health -= damage;
            println!(
                "You attacked the {}, doing {} damage.",
                self.enemy_type, damage
            );
        }

        println!("It has {} health.", self.health);
    }

    fn do_damage_to_player(&self, player: &mut Player, damage: i32, miss_chance: f64) {
        let did_miss = random_bool(miss_chance);
        if did_miss {
            println!("The {} missed you.", self.enemy_type);
        } else {
            player.health -= damage;
            println!(
                "The {} attacked you, doing {} damage.",
                self.enemy_type, damage
            );
        }

        println!("You have {} health.", player.health);
    }

    fn attack_cycle(&mut self, player: &mut Player) {
        println!("The {} attacked you.", self.enemy_type);
        let player_success = loop {
            let u_input = get_attack_input();
            match u_input {
                AttackInput::Punch => self.do_damage_to_enemy(5, 1.0 / 5.0),
            }

            if self.health <= 0 {
                break true;
            }

            thread::sleep(time::Duration::from_millis(500));
            self.do_damage_to_player(player, 10, 2.0 / 5.0);
            if player.health <= 0 {
                break false;
            }
        };
        if player_success {
            println!("You killed the {}", self.enemy_type);
        } else {
            println!("The {} defeated you. Game over.", self.enemy_type);
            exit(0);
        }
    }
}
