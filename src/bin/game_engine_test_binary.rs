use clap::{ Parser, Subcommand };
use game_engine::game::Game;

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
    let card = game_engine::card::generate_card();
    println!("{:?}", card);
}

fn play_game_command() {
    let mut game = game_engine::game::Game::new();
    println!("{:?}", game);
    loop {
        play_game_stage(&mut game);
    }
}

fn play_game_stage(game: &mut Game) {
    use std::io::stdin;
    println!("game.stage: {:?}", game.stage);
    match game.stage {
        game_engine::game::Stage::StartTurn(which_player, _) => {
            let card = game.start_turn(which_player);
            println!("You get card {:?}", card)
        }
        game_engine::game::Stage::AssignLane(which_player, _, _) => {
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
            game.put_card_in_lane(which_player, lane_i);
        }
        game_engine::game::Stage::AssignDamage(_) => { todo!() }
    }
}
