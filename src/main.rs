use api::API;
use std::path::PathBuf;

mod api;
mod endpoints;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Less bloat Star Stable Online Launcher (Fuck electron bloatware)"
)]

struct Args {
    /// The username/email used to log in
    #[arg(short = 'e', long = "email")]
    email: String,

    /// The password used to log in
    #[arg(short = 'p', long = "password")]
    password: String,

    /// The path to the SSO.exe file folder, if you don't know what this means, you should go back to default launcher tbh...
    #[arg(
        short = 'g',
        long = "game_path",
        default_value = "C:/Program Files/Star Stable Online/client"
    )]
    game_path: Option<PathBuf>,
}
fn main() {
    // Collect arguments efficiently...
    let args = Args::parse();
    API::launch_game(args)
}
