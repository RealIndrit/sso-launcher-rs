use std::fs;
use std::path::PathBuf;
use crate::api::{AuthResponse, GameStatus, StarStableApi};
use crate::{DownloadGameArgs, DownloadLauncherArgs};
use anyhow::Error;
use json::{object, JsonValue};
use crate::utils::write_to_file;

const FULL_INSTALL: i8 = 0;
const UPDATE: i8 = 1;
const NON_PATCH_UPDATE: i8 = 2;
const REPAIR: i8 = 3;
const READY: i8 = 4;

pub fn download_launcher(download_args: &DownloadLauncherArgs) -> Result<(), Error> {
    let download = match download_args.download_path.to_owned() {
        None => {
            let path = dirs::home_dir()
                .unwrap()
                .as_path()
                .join("Downloads")
                .join("Star Stable Online Setup.exe");
            StarStableApi::download_official_launcher(path)
        }
        Some(path) => StarStableApi::download_official_launcher(path),
    };

    match download {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::msg(format!("Failed to download launcher: {}", e))),
    }
}

/// Update or download all required game files for given patch
pub fn download_game(auth_response: AuthResponse, game_status: GameStatus, args: &DownloadGameArgs) -> Result<(), Error> {
    let path = &args.install_path.clone().unwrap();
    let version_hash = &args.version.clone();
    let manifest_data = get_local_manifest(path);
    let mut install_type = NON_PATCH_UPDATE;

    match manifest_data {
        Ok(data) => {
            if (data == JsonValue::Null) {
                install_type = FULL_INSTALL;
                match version_hash {
                    Some(version) => {
                        match StarStableApi::get_remote_manifest(version.to_owned()) {
                            Ok(remote_manifest) => {
                                println!("{}", remote_manifest);
                            }
                            Err(e) => return Err(e),
                        }
                    },
                    None => {
                        match StarStableApi::get_remote_manifest(game_status.game_version.clone()) {
                            Ok(remote_manifest) => {
                                println!("{}", remote_manifest);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            } else {
                if let Some(client_version) = data["client"]["version"].as_str() {
                    if game_status.game_version.clone() == client_version {
                        return Ok(());
                    }
                }
            }

            let client_json = object!{ "client" => object!{ "name" => "client", "version" => game_status.game_version.clone() } };
            match store_local_manifest(&path, client_json) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

#[inline(always)]
pub fn get_local_manifest(path: &PathBuf) -> Result<JsonValue, Error> {
    let contents = fs::read_to_string(path.clone().join("manifest.json"));
    match contents {
        Ok(data) => match json::parse(data.as_str()) {
            Ok(json) => Ok(json),
            Err(e) => Err(Error::msg(format!(
                "Could not parse JSON data from file: {}",
                e
            ))),
        },
        Err(err) => {
            if err.raw_os_error().unwrap() == 3 {
                return Ok(JsonValue::Null);
            }

            Err(Error::from(err))
        }
    }
}

#[inline(always)]
pub fn store_local_manifest(path: &PathBuf, data: JsonValue) -> Result<(), Error> {
    match write_to_file(
        &path.clone().join("manifest.json"),
        json::stringify_pretty(data, 4),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err))
    }
}
