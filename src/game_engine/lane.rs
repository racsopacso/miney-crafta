use crate::card::Card;

#[derive(Debug)]
pub struct Lane(Vec<Card>);

impl Lane {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add_to_lane(&mut self, card: Card) {
        self.0.push(card);
    }
}
