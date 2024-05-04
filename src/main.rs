use api::API;
use std::path::PathBuf;
use std::process::exit;

mod api;
mod endpoints;
use anyhow::Error;
use clap::{Parser, ValueEnum};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
struct Args {
    /// The email used to log in
    #[arg(short = 'e', long)]
    email: String,

    /// The password used to log in
    #[arg(short = 'p', long)]
    password: String,

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

    #[arg(
    short = 'a',
    long,
    default_value = None
    )]
    game_arguments: Option<Vec<String>>,
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
    if let Err(e) = do_stuff() {
        eprintln!("{} {}", "error:".bright_red().bold(), e);
        exit(1);
    }
    exit(0);
}

fn do_stuff() -> Result<(), Error> {
    let args = Args::parse();
    match API::get_launch_args(&args) {
        Ok(mut launch_args) => {
            let exe = launch_args.remove(0);
            println!("{:?}", exe);
            std::process::Command::new(exe)
                .args(launch_args)
                .current_dir(&args.install_path.unwrap())
                .spawn()
                .expect("Couldn't start 'SSOClient.exe'!");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
