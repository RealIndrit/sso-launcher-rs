mod api;
mod endpoints;
mod launch;
mod status;
mod update;

use crate::api::StarStableApi;
use crate::launch::launch_game;
use crate::status::status_game;
use crate::update::update_game;
use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::Colorize;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The email used to log in
    #[arg(short = 'e', long)]
    email: String,

    /// The password used to log in
    #[arg(short = 'p', long)]
    password: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launches the game
    Launch(LaunchArgs),

    /// Updates the game, use this if you fail to join a server
    Update(UpdateArgs),

    /// Fetches Server status for the logged in account
    Status,
}

#[derive(Args)]
struct LaunchArgs {
    /// The path to the SSO.exe file folder
    #[arg(
        short = 'i',
        long,
        default_value = "C:/Program Files/Star Stable Online/client"
    )]
    install_path: Option<PathBuf>,

    /// The language the game will be set to
    #[arg(short = 'l', long, default_value = "en", value_enum)]
    language: Option<Language>,

    /// Game arguments sent directly to the game executable (not available by default on official launcher)
    #[arg(
    short = 'a',
    long,
    default_value = None
    )]
    game_arguments: Option<Vec<String>>,
}

#[derive(Args, Debug)]
struct UpdateArgs {
    #[arg(short = 'v', long)]
    version: Option<String>,

    /// The path to the SSO.exe file folder
    #[arg(
        short = 'i',
        long,
        default_value = "C:/Program Files/Star Stable Online/client"
    )]
    install_path: Option<PathBuf>,
}

#[derive(Clone, ValueEnum, Debug)]
#[allow(non_camel_case_types)]
enum Language {
    en,
    sv,
    de,
    es,
    no,
    fr,
    ru,
    nl,
    hu,
    it,
    pl,
    pt,
    fi,
    da,
}

fn main() {
    let cli = Cli::parse();
    let auth_response =
        StarStableApi::login(cli.email.to_owned(), cli.password.to_owned()).unwrap();
    let game_status =
        StarStableApi::get_game_server_data(auth_response.launcher_hash.clone()).unwrap();

    match &cli.command {
        Commands::Launch(args) => {
            if let Err(e) = launch_game(auth_response, args) {
                eprintln!("{}: {}", "error".bright_red().bold(), e);
                exit(1);
            }
        }
        Commands::Update(args) => {
            if let Err(e) = update_game(auth_response, args) {
                eprintln!("{}: {}", "error".bright_red().bold(), e);
                exit(1);
            }
        }
        Commands::Status => {
            if let Err(e) = status_game(game_status) {
                eprintln!("{}: {}", "error".bright_red().bold(), e);
                exit(1);
            }
        }
    }
}
