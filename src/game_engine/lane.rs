use crate::card::{ self, Card };
use anyhow::{ anyhow, Result };

#[derive(Debug)]
pub struct Lane {
    cards: Vec<Card>,
    pub damage_to_deal: u128,
}

impl Lane {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cards: Vec::new(),
            damage_to_deal: 0,
        }
    }
    pub fn add_to_lane(&mut self, card: Card) {
        self.cards.push(card);
    }
    #[must_use]
    pub fn total_damage(&self) -> u128 {
        self.cards
            .iter()
            .map(|card| Into::<u128>::into(card.attack))
            .sum()
    }
    #[must_use]
    pub fn total_defense(&self) -> u128 {
        self.cards
            .iter()
            .map(|card| Into::<u128>::into(card.defense))
            .sum()
    }
    pub fn init_damage_counter(&mut self) {
        self.damage_to_deal = self.total_damage();
    }
    pub fn get_card_mut(&mut self, id: card::Id) -> Result<&mut Card> {
        let matching_cards: Vec<_> = self.cards
            .iter_mut()
            .filter(|card| card.id == id)
            .collect();
        #[allow(clippy::option_if_let_else)]
        match TryInto::<[_; 1]>::try_into(matching_cards) {
            Ok(v) => Ok(v[0]),
            Err(_) => Err(anyhow!("Could not find card {:?}", id)),
        }
    }
    pub fn remove_card(&mut self, id: card::Id) -> Result<()> {
        if let Some(i) = self.cards.iter().position(|c| c.id == id) {
            self.cards.remove(i);
            Ok(())
        } else {
            Err(anyhow!("No such card {:?}", id))
        }
    }
}

impl Default for Lane {
    fn default() -> Self {
        Self::new()
    }
}
