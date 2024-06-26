use crate::{ card::Card, game, lane::Lane };

#[derive(Debug)]
pub struct Player {
    #[allow(dead_code)]
    which: game::players::WhichPlayer,
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
    #[must_use]
    pub fn new(which: game::players::WhichPlayer) -> Self {
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

    pub fn get_lane_by_index_mut(&mut self, lane_i: u8) -> &mut Lane {
        match lane_i {
            0 => &mut self.lane_one,
            1 => &mut self.lane_two,
            2 => &mut self.lane_three,
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
