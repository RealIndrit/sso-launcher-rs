use anyhow::Error;
use crate::api::StarStableApi;
use crate::{endpoints, LaunchArgs};

/// Launches the game using the given auth response.
pub fn launch_game(args: &LaunchArgs) -> Result<(), Error> {
    let path = &args.install_path.clone().unwrap();
    let exe = &path.clone().join("SSOClient.exe");
    if !std::path::Path::new(exe).exists() {
        return Err(Error::msg(
            "No 'SSOClient.exe' is present. Make sure that this path is correct!",
        ));
    }

    let mut launch_args: Vec<String> = vec![];

    launch_args.push(exe.display().to_string());

    match args.language.to_owned() {
        None => {
            launch_args.push("-Language=en".to_string());
        }
        Some(lang) => {
            launch_args.push(format!("-Language={:?}", lang));
        }
    }

    match StarStableApi::login(args.email.to_owned(), args.password.to_owned()) {
        Ok(auth_response) => {
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
        }
        Err(e) => return Err(e),
    };

    match args.game_arguments.clone() {
        None => (),
        Some(game_args) => {
            for game_arg in game_args {
                launch_args.push(format!("-{}", game_arg));
            }

        }
    }

    std::process::Command::new(exe)
        .args(launch_args)
        .current_dir(path)
        .spawn()
        .expect("Couldn't start 'SSOClient.exe'!");

    Ok(())
}