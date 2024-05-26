use crate::card;
use crate::card::Card;
use crate::lane::Lane;
use crate::player::Player;

struct Game {
    lane_one: Lane,
    lane_two: Lane,
    lane_three: Lane,
    players: [Player; 2],
    stage: Stage,
}

#[derive(PartialEq, Eq)]
pub enum WhichPlayer {
    PlayerOne,
    PlayerTwo,
}

enum Stage {
    StartTurn(WhichPlayer),
    AssignLane(WhichPlayer),
    AssignDamage(WhichPlayer),
}

fn forward_stage(stage: Stage) -> Stage {
    match stage {
        Stage::StartTurn(player) => Stage::AssignDamage(player),
        Stage::AssignLane(WhichPlayer::PlayerOne) => Stage::StartTurn(WhichPlayer::PlayerTwo),
        Stage::AssignLane(WhichPlayer::PlayerTwo) => Stage::AssignDamage(WhichPlayer::PlayerOne),
        Stage::AssignDamage(WhichPlayer::PlayerOne) => Stage::AssignDamage(WhichPlayer::PlayerTwo),
        Stage::AssignDamage(WhichPlayer::PlayerTwo) => Stage::StartTurn(WhichPlayer::PlayerOne),
    }
}

impl Game {
    pub fn new() -> Self {
        return Game {
            lane_one: Lane::new(),
            lane_two: Lane::new(),
            lane_three: Lane::new(),
            players: [
                Player::new(WhichPlayer::PlayerOne),
                Player::new(WhichPlayer::PlayerTwo),
            ],
            stage: Stage::StartTurn(WhichPlayer::PlayerOne),
        };
    }

    pub fn start_turn(mut self, which_player: WhichPlayer) -> Result<Card, ()> {
        match self.stage {
            Stage::StartTurn(ref player) => {
                if *player != which_player {
                    return Err(());
                }
            }
            _ => (),
        };
        let card = card::generate_card();
        self.stage = forward_stage(self.stage);
        return Ok(card);
    }
}
