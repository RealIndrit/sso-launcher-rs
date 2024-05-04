use api::API;
use std::path::PathBuf;
use std::process::exit;

mod api;
mod endpoints;
use clap::Parser;
use colored::Colorize;
use anyhow::Error;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Less bloat Star Stable Online Launcher (Fuck electron bloatware)"
)]

struct Args {
    /// The email used to log in
    #[arg(short = 'e', long = "email")]
    email: String,

    /// The password used to log in
    #[arg(short = 'p', long = "password")]
    password: String,

    /// The path to the SSO.exe file folder
    #[arg(
        short = 'i',
        long = "install_path",
        default_value = "C:/Program Files/Star Stable Online/client"
    )]
    install_path: Option<PathBuf>
}

fn main() {
    if let Err(e) = do_stuff() {
        eprintln!("{} {}", "error:".bright_red().bold(), e);
        exit(1);
    }
    exit(0);
}

fn do_stuff() -> Result<(), Error>  {
    let args = Args::parse();
    match API::get_launch_args(&args) {
        Ok(mut launch_args) => {
            let exe = launch_args.remove(0);

            std::process::Command::new(exe)
                .args(launch_args)
                .current_dir(&args.install_path.unwrap())
                .spawn()
                .expect("Couldn't start 'SSOClient.exe'!");
            Ok(())
        },
        Err(e) => Err(e)
    }
}
