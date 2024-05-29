use crate::card::{ Card, DeadCard };
use crate::player::Player;
use crate::card;
use anyhow::{ anyhow, Result };

pub mod i_am_player_witness {
    pub trait W {}
}

pub mod players {
    use super::i_am_player_witness;

    #[derive(Clone, Debug)]
    pub struct PlayerOne;
    impl i_am_player_witness::W for PlayerOne {}
    #[derive(Clone, Debug)]
    pub struct PlayerTwo;
    impl i_am_player_witness::W for PlayerTwo {}
    #[derive(Clone, Copy, Eq, Debug, PartialEq)]
    pub enum WhichPlayer {
        PlayerOne,
        PlayerTwo,
    }
}
use players::{ WhichPlayer, WhichPlayer::{ PlayerOne, PlayerTwo } };

mod i_am_stage_witness {
    pub trait W {
        type T: Clone;
        fn get_stage_data(&self) -> &Self::T;
    }
}

pub mod stages {
    use std::marker::PhantomData;
    use super::{ i_am_player_witness, players::WhichPlayer };
    use crate::card::Card;
    use super::i_am_stage_witness::W;

    #[derive(Clone, Debug)]
    pub struct StartTurn<T>(pub WhichPlayer, pub Vec<u8>, pub PhantomData<T>);
    impl<T> W for StartTurn<T> where T: Clone {
        type T = StartTurn<T>;
        fn get_stage_data(&self) -> &Self::T {
            self
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssignLane<T: i_am_player_witness::W>(
        pub WhichPlayer,
        pub Card,
        pub Vec<u8>,

        pub PhantomData<T>,
    );
    impl<T> W for AssignLane<T> where T: i_am_player_witness::W + Clone {
        type T = AssignLane<T>;
        fn get_stage_data(&self) -> &Self::T {
            self
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssignDamage;
    impl W for AssignDamage {
        type T = AssignDamage;

        fn get_stage_data(&self) -> &Self::T {
            self
        }
    }
}

#[derive(Debug)]
pub struct Game<T: i_am_stage_witness::W> {
    players: [Player; 2],
    phantom: T,
}

#[must_use]
pub const fn other_player(which_player: WhichPlayer) -> WhichPlayer {
    match which_player {
        PlayerOne => PlayerTwo,
        PlayerTwo => PlayerOne,
    }
}

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct AssignDamageSpec {
    pub to_player: WhichPlayer,
    pub lane_i: u8,
    pub card_id: card::Id,
}

#[must_use]
pub fn new() -> Game<stages::StartTurn<players::PlayerOne>> {
    Game {
        players: [Player::new(PlayerOne), Player::new(PlayerTwo)],
        phantom: stages::StartTurn(PlayerOne, Vec::new(), std::marker::PhantomData),
    }
}

impl<T> Game<T> where T: i_am_stage_witness::W + Clone {
    fn get_stage_data(&self) -> T {
        self.phantom.clone()
    }

    // bhack: I should be more consistent with the _mut naming convention
    fn get_player(&mut self, which_player: WhichPlayer) -> &mut Player {
        match which_player {
            PlayerOne => &mut self.players[0],
            PlayerTwo => &mut self.players[1],
        }
    }
}

impl<T> Game<stages::StartTurn<T>> where T: i_am_player_witness::W + Clone {
    pub fn start_turn(&mut self) -> Card {
        let card = card::generate();
        card
    }

    pub fn forward_stage(self, card: Card) -> Game<stages::AssignLane<T>> {
        let stages::StartTurn(player, lanes, _) = self.get_stage_data();
        Game::<stages::AssignLane<T>> {
            players: self.players,
            phantom: stages::AssignLane(player, card, lanes, std::marker::PhantomData),
        }
    }
}

impl<T> Game<stages::AssignLane<T>> where T: i_am_player_witness::W + Clone {
    pub fn put_card_in_lane(&mut self, lane_i: u8) {
        let stages::AssignLane(which_player, card, _, _) = self.get_stage_data();
        let player = self.get_player(which_player);
        player.put_card_in_lane(lane_i, card);
    }
}

impl Game<stages::AssignLane<players::PlayerOne>> {
    pub fn forward_stage(mut self, lane_i: u8) -> Game<stages::StartTurn<players::PlayerTwo>> {
        let stages::AssignLane(which_player, card, lanes, _) = self.get_stage_data();
        let mut lanes = lanes.clone();
        let lane_i = lane_i;
        lanes.push(lane_i);
        Game::<stages::StartTurn<players::PlayerTwo>> {
            players: self.players,
            phantom: stages::StartTurn(PlayerTwo, lanes, std::marker::PhantomData),
        }
    }
}
impl Game<stages::AssignLane<players::PlayerTwo>> {
    pub fn forward_stage(mut self, lane_i: u8) -> Game<stages::AssignDamage> {
        let stages::AssignLane(_, _, lanes_to_exclude, _) = self.get_stage_data();
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
        Game::<stages::AssignDamage> {
            players: self.players,
            phantom: stages::AssignDamage {},
        }
    }
}

impl Game<stages::AssignDamage> {
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

    pub fn forward_stage(self) -> Game<stages::StartTurn<players::PlayerOne>> {
        Game {
            players: self.players,
            phantom: stages::StartTurn(PlayerOne, Vec::new(), std::marker::PhantomData),
        }
    }
}
