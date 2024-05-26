use crate::card;
use crate::card::Card;
use crate::lane::Lane;
use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    players: [Player; 2],
    pub stage: Stage,
}

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum WhichPlayer {
    PlayerOne,
    PlayerTwo,
}

#[derive(Clone, Copy, Debug)]
pub enum Stage {
    StartTurn(WhichPlayer),
    AssignLane(WhichPlayer, Card),
    AssignDamage(WhichPlayer),
}

fn forward_stage(stage: Stage, card: Option<Card>) -> Stage {
    match stage {
        Stage::StartTurn(player) => {
            let card = card.unwrap();
            Stage::AssignLane(player, card)
        }
        Stage::AssignLane(WhichPlayer::PlayerOne, _) => Stage::StartTurn(WhichPlayer::PlayerTwo),
        Stage::AssignLane(WhichPlayer::PlayerTwo, _) => Stage::AssignDamage(WhichPlayer::PlayerOne),
        Stage::AssignDamage(WhichPlayer::PlayerOne) => Stage::AssignDamage(WhichPlayer::PlayerTwo),
        Stage::AssignDamage(WhichPlayer::PlayerTwo) => Stage::StartTurn(WhichPlayer::PlayerOne),
    }
}

impl Game {
    pub fn new() -> Self {
        return Game {
            players: [
                Player::new(WhichPlayer::PlayerOne),
                Player::new(WhichPlayer::PlayerTwo),
            ],
            stage: Stage::StartTurn(WhichPlayer::PlayerOne),
        };
    }

    pub fn start_turn(&mut self, which_player: WhichPlayer) -> Card {
        match self.stage {
            Stage::StartTurn(ref player) => {
                if *player != which_player {
                    panic!("oopsie")
                }
            }
            _ => panic!("oopsie"),
        };
        let card = card::generate_card();
        self.stage = forward_stage(self.stage, Some(card));
        return card.clone();
    }

    fn get_player(&mut self, which_player: WhichPlayer) -> &mut Player {
        match which_player {
            WhichPlayer::PlayerOne => &mut self.players[0],
            WhichPlayer::PlayerTwo => &mut self.players[1],
        }
    }

    pub fn put_card_in_lane(&mut self, which_player: WhichPlayer, lane_i: u8) {
        let card = match self.stage {
            Stage::AssignLane(ref player, card) => {
                if *player != which_player {
                    panic!("oopsie")
                };
                card
            }
            _ => panic!("oopsie!"),
        };
        let player = self.get_player(which_player);
        player.put_card_in_lane(lane_i, card);
        self.stage = forward_stage(self.stage, None);
    }
}
