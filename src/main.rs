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
        short = 'g',
        long = "game_path",
        default_value = "C:/Program Files/Star Stable Online/client"
    )]
    game_path: Option<PathBuf>
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
    match API::get_launch_args(args) {
        Ok(mut launch_args) => {
            let exe = launch_args.remove(0);

            // TODO: cleanup, this is a mentally disadvantaged way of doing it
            let arg0 = launch_args.get(0).unwrap();
            let arg1 = launch_args.get(1).unwrap();
            let arg2 = launch_args.get(2).unwrap();
            let arg3 = launch_args.get(3).unwrap();
            let arg4 = launch_args.get(4).unwrap();
            let arg5 = launch_args.get(5).unwrap();
            let arg6 = launch_args.get(6).unwrap();
            let arg7 = launch_args.get(7).unwrap();

            // "C:\Users\johan\AppData\Local\Microsoft\WindowsApps\WinDbgX.exe"  -T "Star Stable Online Debug"
            // D:\\StarStableEnt\\Development\\game\\client\\SSOClient.exe
            // "-Language=\"sv\" -NetworkUserId=\"58948879\" -MetricsServer=\"https://metrics.starstable.com/metric/v1/metrics\" -MetricsGroup=\"[1]\" -LoginQueueToken=\"9bd1a5c8-892f-41a1-bab4-ad7f16618cd0\" -NetworkLauncherHash=\"4a8d95dc-8150-43b1-ac54-8d2005d333b4\" -ProjectUserDataPath=\"D:\\StarStableEnt\\Development\\game\\client\" -NetworkLauncherServer=\"https://launcher-proxy.starstable.com\""
            let argument_string = format!("{arg0} {arg1} {arg2} {arg3} {arg4} {arg5} {arg6} {arg7}");
            println!("{:?}", argument_string);

            std::process::Command::new(exe)
                .arg(argument_string)
                .spawn()
                .expect("Couldn't start 'SSOClient.exe'!");
            Ok(())
        },
        Err(e) => Err(e)
    }
}
