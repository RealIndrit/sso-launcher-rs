use crate::api::{AuthResponse, GameStatus};
use crate::{endpoints, LaunchArgs};
use anyhow::Error;

/// Launches the game using the given auth response.
pub fn launch_game(auth_response: AuthResponse, game_status: GameStatus, args: &LaunchArgs) -> Result<(), Error> {
    // Path to client folder within the installation
    let path = &args.install_path.clone().unwrap().join("client");
    let exe = &path.clone().join("SSOClient.exe");
    if !std::path::Path::new(exe).exists() {
        return Err(Error::msg(
            "No 'SSOClient.exe' is present. Make sure that this path is correct! Use --help for more info.",
        ));
    }

    // Do some sanity checks before trying to launch game
    if(game_status.update_in_progress == true){
        return Err(Error::msg(format!("Game server '{}' undergoing update '{}', please try again later", game_status.friendly_name, game_status.game_version)));
    }

    if(game_status.online != true && game_status.update_in_progress != true){
        return Err(Error::msg(format!("Game server '{}' is not available at the time for unknown reason, please try again later. For more information see Star Stable Onlines's website", game_status.friendly_name)))
    }

    // Sanity checks passed, build argument structure being passed to game executable
    let mut launch_args: Vec<String> = vec![];

    match args.language.to_owned() {
        None => {
            launch_args.push("-Language=en".to_string());
        }
        Some(lang) => {
            launch_args.push(format!("-Language={:?}", lang));
        }
    }

    launch_args.push(format!("-NetworkUserId={}", auth_response.user_id));
    launch_args.push(format!("-MetricsServer={}", endpoints::METRICS));
    launch_args.push(format!("-MetricsGroup={}", "[1]"));
    launch_args.push(format!("-LoginQueueToken={}", auth_response.queue_token));
    launch_args.push(format!(
        "-NetworkLauncherHash={}",
        auth_response.launcher_hash
    ));
    launch_args.push(format!(
        "-ProjectUserDataPath={}",
        &path.clone().to_string_lossy()
    ));
    launch_args.push(format!(
        "-NetworkLauncherServer={}",
        endpoints::LAUNCHER_PROXY
    ));

    match args.game_arguments.clone() {
        None => (),
        Some(game_args) => {
            for game_arg in game_args {
                launch_args.push(format!("-{}", game_arg));
            }
        }
    }

    println!("Launching game with following arguments: {:?}", launch_args);
    // WE BALLLL
    // NOTE: .current_dir() is important AF as the executable uses that to find its relative position to asset files LOL.
    std::process::Command::new(exe)
        .args(launch_args)
        .current_dir(path)
        .spawn()
        .expect("Couldn't start 'SSOClient.exe'!");

    Ok(())
}
