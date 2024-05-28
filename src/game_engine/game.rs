use crate::card::{ Card, DeadCard };
use crate::player::Player;
use crate::card;
use anyhow::{ anyhow, Result };

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
use WhichPlayer::{ PlayerOne, PlayerTwo };

#[must_use]
pub const fn other_player(which_player: WhichPlayer) -> WhichPlayer {
    match which_player {
        PlayerOne => PlayerTwo,
        PlayerTwo => PlayerOne,
    }
}

#[derive(Clone, Debug)]
pub enum Stage {
    StartTurn(WhichPlayer, Vec<u8>),
    AssignLane(WhichPlayer, Card, Vec<u8>),
    AssignDamage(),
}
use Stage::{ AssignDamage, AssignLane, StartTurn };

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct AssignDamageSpec {
    pub to_player: WhichPlayer,
    pub lane_i: u8,
    pub card_id: card::Id,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            players: [Player::new(PlayerOne), Player::new(PlayerTwo)],
            stage: StartTurn(PlayerOne, Vec::new()),
        }
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
        let card = card::generate();
        self.stage = self.forward_stage(Some(card), None);
        card
    }

    // bhack: I should be more consistent with the _mut naming convention
    fn get_player(&mut self, which_player: WhichPlayer) -> &mut Player {
        match which_player {
            PlayerOne => &mut self.players[0],
            PlayerTwo => &mut self.players[1],
        }
    }

    pub fn put_card_in_lane(&mut self, which_player: WhichPlayer, lane_i: u8) {
        let card = match self.stage.clone() {
            AssignLane(ref player, card, _) => {
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

    //bhack: make this simultaneous
    pub fn apply_damage(
        &mut self,
        which_player: WhichPlayer,
        assign_spec: AssignDamageSpec,
        amount: u8
    ) -> Result<card::OkOrDead<()>> {
        let player_damaging = self.get_player(which_player);
        let AssignDamageSpec { to_player, lane_i, card_id } = assign_spec;
        let sending_lane = player_damaging.get_lane_by_index_mut(lane_i);
        if sending_lane.damage_to_deal < amount.into() {
            Err(anyhow!("dealing more damage than available!"))?;
        } else {
            Ok::<(), anyhow::Error>(())?;
        }
        let player_to_damage = self.get_player(to_player);
        let receiving_lane = player_to_damage.get_lane_by_index_mut(lane_i);
        let card = receiving_lane.get_card_mut(card_id)?;
        match card.damage(amount) {
            Ok(()) => Ok(Ok(())),
            Err(DeadCard {}) => {
                receiving_lane.remove_card(card_id)?;
                Ok(Err(DeadCard {}))
            }
        }
    }

    // bhack: it isn't great that we're using options to cover up a bad function
    // signature
    pub fn forward_stage(&mut self, card: Option<Card>, lane_i: Option<u8>) -> Stage {
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
                let all_lanes = vec![0, 1, 2];
                let lanes_to_damage: Vec<_> = all_lanes
                    .into_iter()
                    .filter(|i| !lanes_to_exclude.contains(i))
                    .collect();
                self.players.iter_mut().for_each(|player: &mut Player|
                    // bhack: we could invert these loops and clone less, but I'm going to be a maverick
                    // and waste a couple bytes of memory.
                    {
                        lanes_to_damage
                            .clone()
                            .into_iter()
                            .for_each(|lane_i| {
                                let lane = player.get_lane_by_index_mut(lane_i);
                                lane.init_damage_counter();
                            });
                        lanes_to_exclude
                            .clone()
                            .into_iter()
                            .for_each(|lane_i| {
                                let lane = player.get_lane_by_index_mut(lane_i);
                                lane.damage_to_deal = 0;
                            });
                    }
                );
                AssignDamage()
            }
            AssignDamage() => StartTurn(PlayerOne, Vec::new()),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
