use clap::{ Parser, Subcommand };
use game_engine::card::DeadCard;
use game_engine::game::{ i_am_player_witness, Game };

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args_ {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Card(card::Args_),
    Game(game::Args_),
}

mod card {
    use clap::{ Args, Subcommand };
    #[derive(Args)]
    pub struct Args_ {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        // ???
        Generate(generate::Args_),
    }

    pub mod generate {
        use clap::Args;
        #[derive(Args)]
        pub struct Args_ {}
    }
}

pub mod game {
    use clap::{ Args, Subcommand };
    #[derive(Args)]
    pub struct Args_ {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        // ???
        Play(play::Args_),
    }

    pub mod play {
        use clap::Args;
        #[derive(Args)]
        pub struct Args_ {}
    }
}

fn main() {
    let cli = Args_::parse();

    match &cli.command {
        Commands::Card(card::Args_ { command: card::Commands::Generate(args) }) => {
            let _ = args;
            generate_card_command();
        }
        Commands::Game(game::Args_ { command: game::Commands::Play(args) }) => {
            let _ = args;
            play_game_command()
        }
    }
}

fn generate_card_command() {
    println!("Generating card!");
    let card = game_engine::card::generate();
    println!("{:?}", card);
}

fn play_game_command() -> ! {
    let game = game_engine::game::new();
    println!("{:?}", game);
    PlayGameStageStartGame::<game_engine::game::players::PlayerOne>(
        std::marker::PhantomData
    ).play_game_stage_start_turn(game);
}

fn play_game_stage_start_turn_common<T>(
    game: &mut Game<game_engine::game::stages::StartTurn<T>>
) -> game_engine::card::Card
    where T: game_engine::game::i_am_player_witness::W + Clone
{
    println!("game.stage: {:#?}", game.stage);
    let card = game.start_turn();
    println!("You get card {:?}", card);
    card
}

struct PlayGameStageStartGame<T>(std::marker::PhantomData<T>);
impl PlayGameStageStartGame<game_engine::game::players::PlayerOne> {
    pub fn play_game_stage_start_turn(
        self,
        mut game: Game<game_engine::game::stages::StartTurn<game_engine::game::players::PlayerOne>>
    ) -> ! {
        let card = play_game_stage_start_turn_common(&mut game);
        let game = game.forward_stage(card);
        PlayGameStageAssignLane::<game_engine::game::players::PlayerOne>(
            std::marker::PhantomData
        ).play_game_stage_assign_lane(game);
    }
}
impl PlayGameStageStartGame<game_engine::game::players::PlayerTwo> {
    pub fn play_game_stage_start_turn(
        self,
        mut game: Game<game_engine::game::stages::StartTurn<game_engine::game::players::PlayerTwo>>
    ) -> ! {
        let card = play_game_stage_start_turn_common(&mut game);
        let game = game.forward_stage(card);
        PlayGameStageAssignLane::<game_engine::game::players::PlayerTwo>(
            std::marker::PhantomData
        ).play_game_stage_assign_lane(game);
    }
}

fn play_game_stage_assign_lane_common<T>(
    game: &mut Game<game_engine::game::stages::AssignLane<T>>
) -> u8
    where T: i_am_player_witness::W + Clone
{
    use std::io::stdin;
    let mut s = String::new();
    // bhack: I feel like there must be a better way of writing "get a u8"
    stdin().read_line(&mut s).expect("???");
    println!("{}", s);
    let lane_i = match s.trim().parse::<u8>() {
        Ok(v) => {
            if v > 3 {
                panic!("go away!");
            }
            v
        }
        Err(_) => panic!("go away!"),
    };
    game.put_card_in_lane(lane_i);
    lane_i
}

struct PlayGameStageAssignLane<T>(std::marker::PhantomData<T>);
impl PlayGameStageAssignLane<game_engine::game::players::PlayerOne> {
    fn play_game_stage_assign_lane(
        self,
        mut game: Game<game_engine::game::stages::AssignLane<game_engine::game::players::PlayerOne>>
    ) -> ! {
        let lane_i = play_game_stage_assign_lane_common(&mut game);
        let game = game.forward_stage(lane_i);
        PlayGameStageStartGame::<game_engine::game::players::PlayerTwo>(
            std::marker::PhantomData
        ).play_game_stage_start_turn(game);
    }
}
impl PlayGameStageAssignLane<game_engine::game::players::PlayerTwo> {
    fn play_game_stage_assign_lane(
        self,
        mut game: Game<game_engine::game::stages::AssignLane<game_engine::game::players::PlayerTwo>>
    ) -> ! {
        let lane_i = play_game_stage_assign_lane_common(&mut game);
        let game = game.forward_stage(lane_i);
        play_game_stage_apply_damage(game);
    }
}

fn play_game_stage_apply_damage(mut game: Game<game_engine::game::stages::AssignDamage>) -> ! {
    use std::io::stdin;
    use game_engine::game::AssignDamageSpec;
    println!("Entering assign damage loop. Game: {:#?}", game);
    'readloop: loop {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("???");
        let s: Vec<_> = s.trim().split(" ").collect();
        match s[..] {
            ["q"] | ["quit"] | ["exit"] => {
                println!("breaking {:#?}", game.stage);
                break 'readloop;
            }
            | ["a", to_player, lane_i, card_id, amount]
            | ["assign", to_player, lane_i, card_id, amount]
            | ["attack", to_player, lane_i, card_id, amount] => {
                let to_player = match to_player {
                    "0" => game_engine::game::players::WhichPlayer::PlayerOne,
                    "1" => game_engine::game::players::WhichPlayer::PlayerTwo,
                    _ => panic!("!"),
                };
                let lane_i = lane_i.parse::<u8>().unwrap();
                let amount = amount.parse::<u8>().unwrap();
                let card_id = card_id.try_into().unwrap();
                let assigning_player = game_engine::game::other_player(to_player);
                match
                    game
                        .apply_damage(
                            assigning_player,
                            AssignDamageSpec { to_player, lane_i, card_id },
                            amount
                        )
                        .unwrap()
                {
                    Ok(()) => (),
                    Err(DeadCard {}) => println!("Card ({:?}) killed!", card_id),
                }
            }
            _ => println!("unrecongnised command: {:#?}", s),
        }
    }
    let game = game.forward_stage();
    PlayGameStageStartGame::<game_engine::game::players::PlayerOne>(
        std::marker::PhantomData
    ).play_game_stage_start_turn(game)
}
