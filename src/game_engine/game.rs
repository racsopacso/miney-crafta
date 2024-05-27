use crate::card::Card;
use crate::player::Player;
use crate::{ card, lane };

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

#[derive(Clone, Debug)]
pub enum Stage {
    StartTurn(WhichPlayer, Vec<u8>),
    AssignLane(WhichPlayer, Card, Vec<u8>),
    AssignDamage(Vec<u8>),
}

use Stage::*;
use WhichPlayer::*;

impl Game {
    pub fn new() -> Self {
        return Game {
            players: [Player::new(PlayerOne), Player::new(PlayerTwo)],
            stage: StartTurn(PlayerOne, Vec::new()),
        };
    }

    pub fn start_turn(&mut self, which_player: WhichPlayer) -> Card {
        match self.stage {
            StartTurn(ref player, _) => {
                if *player != which_player {
                    panic!("oopsie");
                }
            }
            _ => panic!("oopsie"),
        }
        let card = card::generate_card();
        self.stage = self.forward_stage(Some(card), None);
        return card.clone();
    }

    fn get_player(&mut self, which_player: WhichPlayer) -> &mut Player {
        match which_player {
            PlayerOne => &mut self.players[0],
            PlayerTwo => &mut self.players[1],
        }
    }

    pub fn put_card_in_lane(&mut self, which_player: WhichPlayer, lane_i: u8) {
        let card = match self.stage.clone() {
            AssignLane(ref player, card, lanes) => {
                if *player != which_player {
                    panic!("oopsie");
                }
                card
            }
            _ => panic!("oopsie!"),
        };
        let player = self.get_player(which_player);
        player.put_card_in_lane(lane_i, card);
        self.stage = self.forward_stage(None, Some(lane_i));
    }

    pub fn apply_damage(&mut self, which_player: WhichPlayer, lane: u8, card: card::Id) {}

    // bhack: it isn't great that we're using options to cover up a bad function
    // signature
    fn forward_stage(&mut self, card: Option<Card>, lane_i: Option<u8>) -> Stage {
        match self.stage.clone() {
            StartTurn(player, lanes) => {
                let card = card.unwrap();
                AssignLane(player, card, lanes)
            }
            AssignLane(PlayerOne, _, mut lanes) => {
                let lane_i = lane_i.unwrap();
                lanes.push(lane_i);
                StartTurn(PlayerTwo, lanes)
            }
            AssignLane(PlayerTwo, _, lanes_to_exclude) => {
                let all_lanes = vec![1, 2, 3];
                let lanes_to_damage = all_lanes
                    .into_iter()
                    .filter(|i| !lanes_to_exclude.contains(i))
                    .collect();
                AssignDamage(lanes_to_damage)
            }
            AssignDamage(_) => StartTurn(PlayerOne, Vec::new()),
        }
    }
}
