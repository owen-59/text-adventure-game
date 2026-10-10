use std::any::Any;

use dyn_clone::DynClone;
use rand::seq::IndexedRandom;

use crate::input::InputMessage;

pub type Item = Box<dyn InventoryItem>;

pub trait InventoryItem: DynClone + Any {
    fn display(&self) -> &'static str;
    fn should_show(&self) -> bool;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

dyn_clone::clone_trait_object!(InventoryItem);

#[derive(Clone)]
pub struct Weapon {
    pub damage: i32,
    pub name: &'static str,
    pub should_show: bool,
    pub message: InputMessage,
    pub key: char,
}

impl InventoryItem for Weapon {
    fn display(&self) -> &'static str {
        self.name
    }
    fn should_show(&self) -> bool {
        self.should_show
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Weapon {
    pub fn new_fist() -> Self {
        Self {
            damage: 5,
            name: "fist",
            should_show: false,
            message: InputMessage::Punch,
            key: 'p',
        }
    }

    pub fn new_sword() -> Self {
        Self {
            damage: 15,
            name: "sword",
            should_show: true,
            message: InputMessage::Sword,
            key: 's',
        }
    }
}

pub fn get_rand_item() -> Option<Box<dyn InventoryItem>> {
    let avail: Vec<Item> = vec![Box::new(Weapon::new_sword())];
    avail.choose(&mut rand::rng()).cloned() as Option<Box<dyn InventoryItem>>
}
