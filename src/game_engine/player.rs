use crate::game;

pub struct Player {
    which: game::WhichPlayer,
}

impl Player {
    pub fn new(which: game::WhichPlayer) -> Self {
        return Player { which };
    }
}
