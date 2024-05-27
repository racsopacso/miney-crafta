use crate::{card::Card, game, lane::Lane};

#[derive(Debug)]
pub struct Player {
    which: game::WhichPlayer,
    health: u8,
    lane_one: Lane,
    lane_two: Lane,
    lane_three: Lane,
}

pub enum OkOrDead {
    Ok,
    Dead,
}

impl Player {
    pub fn new(which: game::WhichPlayer) -> Self {
        Self {
            which,
            health: 10,
            lane_one: Lane::new(),
            lane_two: Lane::new(),
            lane_three: Lane::new(),
        }
    }

    pub fn put_card_in_lane(&mut self, lane_i: u8, card: Card) {
        match lane_i {
            0 => Lane::add_to_lane(&mut self.lane_one, card),
            1 => Lane::add_to_lane(&mut self.lane_two, card),
            2 => Lane::add_to_lane(&mut self.lane_three, card),
            _ => panic!("oopsie"),
        }
    }

    pub fn reduce_health(&mut self, by: u8) -> OkOrDead {
        if self.health > by {
            self.health -= by;
            OkOrDead::Ok
        } else {
            OkOrDead::Dead
        }
    }
}
