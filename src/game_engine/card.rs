use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    attack: u8,
    defense: u8,
}

pub fn generate_card() -> Card {
    let attack = random::<u8>() % 5;
    let defense = random::<u8>() % 5;
    let card = Card { attack, defense };
    return card;
}
