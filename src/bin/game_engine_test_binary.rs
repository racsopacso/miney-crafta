use game_engine;

use clap::{Parser, Subcommand};

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
}

mod card {
    use clap::{Args, Subcommand};
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

fn main() {
    let cli = Args_::parse();

    match &cli.command {
        Commands::Card(card::Args_ {
            command: card::Commands::Generate(args),
        }) => {
            let _ = args;
            generate_card_command();
        }
    }
}

fn generate_card_command() {
    println!("Generating card!");
    game_engine::card_game_function();
}
