use crate::card::Card;

pub struct Lane(Vec<Card>, Vec<Card>);

impl Lane {
    pub fn new() -> Self {
        Self(Vec::new(), Vec::new())
    }
    // bhack: This is probably a bad interface
    pub fn add_to_lane_one(mut self, card: Card) {
        self.0.push(card);
    }
    pub fn add_to_lane_two(mut self, card: Card) {
        self.1.push(card);
    }
}
