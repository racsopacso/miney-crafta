use rand::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub id: Id,
    pub attack: u8,
    pub defense: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Id(Uuid);

pub fn generate_card() -> Card {
    let id = Id(Uuid::now_v7());
    let attack = random::<u8>() % 5;
    let defense = random::<u8>() % 5;
    let card = Card { id, attack, defense };
    return card;
}
