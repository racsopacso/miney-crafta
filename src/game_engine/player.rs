use crate::{game, lane::Lane};

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
}
