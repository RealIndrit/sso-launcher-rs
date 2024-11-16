use crate::api::{AuthResponse, GameStatus};
use crate::update::get_local_manifest;
use crate::{endpoints, LaunchArgs};
use anyhow::Error;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;

/// Launches the game using exe path, cwd, arguments and debug flag.
fn _launch_game(
    exe: &PathBuf,
    launch_args: &Vec<String>,
    cwd: &Path,
    debug: bool,
) -> Result<(), Error> {
    match std::process::Command::new(exe)
        .args(launch_args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => Ok({
            if debug.clone() {
                let mut stdout = child.stdout.take().expect("Failed to capture stdout");
                let mut stderr = child.stderr.take().expect("Failed to capture stderr");

                // Create a thread to handle stdout
                let stdout_thread = std::thread::spawn(move || {
                    let mut buffer = [0; 1024]; // Read in chunks of 1024 bytes
                    let mut handle = io::stdout();

                    while let Ok(bytes_read) = stdout.read(&mut buffer) {
                        if bytes_read == 0 {
                            break;
                        }
                        handle
                            .write_all(&buffer[..bytes_read])
                            .expect("Failed to write to stdout");
                    }
                });

                // Create a thread to handle stderr
                let stderr_thread = std::thread::spawn(move || {
                    let mut buffer = [0; 1024]; // Read in chunks of 1024 bytes
                    let mut handle = io::stderr();

                    while let Ok(bytes_read) = stderr.read(&mut buffer) {
                        if bytes_read == 0 {
                            break;
                        }
                        handle
                            .write_all(&buffer[..bytes_read])
                            .expect("Failed to write to stderr");
                    }
                });

                // Wait for the child process to finish
                let status = child.wait().expect("Failed to wait on child process");
                println!("Child process exited with status: {}", status);

                // Wait for the threads to finish
                stdout_thread.join().expect("Failed to join stdout thread");
                stderr_thread.join().expect("Failed to join stderr thread");
            } else {
                return Ok(());
            }
        }),
        Err(e) => Err(Error::msg(format!(
            "Couldn't start '{}'!: {}",
            &exe.display(),
            e
        ))),
    }
}

/// Launches the game using the given auth response.
pub fn launch_game(
    auth_response: AuthResponse,
    game_status: GameStatus,
    args: &LaunchArgs,
) -> Result<(), Error> {
    // Path to client folder within the installation
    let path = &args.install_path.clone().unwrap().join("client");
    let exe = &path.clone().join("SSOClient.exe");
    if !std::path::Path::new(exe).exists() {
        return Err(Error::msg(
            "No 'SSOClient.exe' is present. Make sure that this path is correct! Use --help for more info.",
        ));
    }

    // Do some sanity checks before trying to launch game
    if game_status.update_in_progress == true {
        return Err(Error::msg(format!(
            "Game server '{}' undergoing update to version '{}', please try again later",
            game_status.friendly_name, game_status.game_version
        )));
    }

    if game_status.online != true && game_status.update_in_progress != true {
        return Err(Error::msg(format!("Game server '{}' is not available at the time for unknown reason, please try again later. For more information see Star Stable Onlines's website", game_status.friendly_name)));
    }

    let mut manifest = get_local_manifest(&args.install_path.clone().unwrap()).unwrap();
    let local_gameversion = manifest["client"].take()["version"].take().to_string();
    if game_status.game_version != local_gameversion {
        return Err(Error::msg(format!(
            "Game server '{}' is not the same version '{}' as installed version '{}', cannot join!",
            game_status.friendly_name, game_status.game_version, local_gameversion
        )));
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
    let debug = args.debug.to_owned();

    match args.ngfx_launch_path.to_owned() {
        None => {
            println!("Launching normal instance of game...");
            println!(
                "Launching game with following arguments: {}",
                &launch_args.join(" ")
            );
            _launch_game(&exe, &launch_args, &path, debug)
        }

        Some(ngfx_path) => {
            let mut ngfx_launch_args: Vec<String> = vec![];
            ngfx_launch_args.push(format!("--activity={}", "Frame Debugger"));
            ngfx_launch_args.push(format!("--platform={}", "Windows"));
            ngfx_launch_args.push(format!("--dir=\"{}\"", &path.display()));
            ngfx_launch_args.push(format!("--exe=\"{}\"", &exe.display()));
            ngfx_launch_args.push(format!("--args={}", &launch_args.join(" ")));
            ngfx_launch_args.push(format!("{}", "--verbose"));
            ngfx_launch_args.push(format!("{}", "--launch-detached"));

            let ngfx_exe = &ngfx_path.join("ngfx.exe");
            println!("Launching NGFX instance of game...");
            println!(
                "Launching game with following arguments: {}",
                ngfx_launch_args.clone().join(" ")
            );
            _launch_game(&ngfx_exe, &ngfx_launch_args, &path, debug)
        }
    }
}
