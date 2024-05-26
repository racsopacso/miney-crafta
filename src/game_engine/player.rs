use crate::{card::Card, game, lane::Lane};

#[derive(Debug)]
pub struct Player {
    which: game::WhichPlayer,
    lane_one: Lane,
    lane_two: Lane,
    lane_three: Lane,
}

impl Player {
    pub fn new(which: game::WhichPlayer) -> Self {
        return Player {
            which,
            lane_one: Lane::new(),
            lane_two: Lane::new(),
            lane_three: Lane::new(),
        };
    }

    pub fn put_card_in_lane(&mut self, lane_i: u8, card: Card) {
        match lane_i {
            0 => Lane::add_to_lane(&mut self.lane_one, card),
            1 => Lane::add_to_lane(&mut self.lane_two, card),
            2 => Lane::add_to_lane(&mut self.lane_three, card),
            _ => panic!("oopsie"),
        }
    }
}
