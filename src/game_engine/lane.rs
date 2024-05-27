use crate::card::Card;

#[derive(Debug)]
pub struct Lane {
    cards: Vec<Card>,
    damage_to_deal: u128,
}

impl Lane {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            damage_to_deal: 0,
        }
    }
    pub fn add_to_lane(&mut self, card: Card) {
        self.cards.push(card)
    }
    pub fn total_damage(&self) -> u128 {
        self.cards
            .iter()
            .map(|card| TryInto::<u128>::try_into(card.attack).unwrap())
            .sum()
    }
    pub fn total_defense(&self) -> u128 {
        self.cards
            .iter()
            .map(|card| TryInto::<u128>::try_into(card.defense).unwrap())
            .sum()
    }
    pub fn init_damage_counter(&mut self) {
        self.damage_to_deal = self.total_damage();
    }
}
