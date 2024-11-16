use crate::{endpoints, utils};
use anyhow::Error;
use json::JsonValue;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Implementation of the `launcher-proxy` API.
#[allow(clippy::upper_case_acronyms)]
pub struct StarStableApi;

/// Auth response.
#[derive(Debug)]
pub struct AuthResponse {
    /// The users Account ID.
    pub(crate) user_id: String,

    /// The users Launcher Hash, aka. Auth Token.
    pub(crate) launcher_hash: String,

    /// The queue token for the user.
    pub(crate) queue_token: String,
}

#[derive(Debug)]
pub struct GameStatus {
    /// id of the server
    pub(crate) id: i16,

    /// region_id of the server
    pub(crate) region_id: i8,

    /// internal name for server
    pub(crate) name: String,

    /// public name for server
    pub(crate) friendly_name: String,

    /// online status for server
    pub(crate) online: bool,

    /// is server down for game updates?
    pub(crate) update_in_progress: bool,

    /// icon, we're never going to use this lol
    pub(crate) icon_url: String,

    /// response code, follows standard return code format?
    pub(crate) message_code: i16,

    /// current active version on the server
    pub(crate) game_version: String,
}

impl StarStableApi {
    // /**
    //  * When Star Stable entertainment decides to fix their shit, we will implement it here
    //  * but at the time they have not updated the repo tag since 2021
    //  *
    //  * {
    //  *   versionInfo: {
    //  *     version: '2.9.13',
    //  *     files: [ [Object] ],
    //  *     path: 'Star Stable Online Setup 2.9.13.exe',
    //  *     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
    //  *     releaseDate: '2021-12-15T09:19:31.282Z'
    //  *   },
    //  *   updateInfo: {
    //  *     version: '2.9.13',
    //  *     files: [ [Object] ],
    //  *     path: 'Star Stable Online Setup 2.9.13.exe',
    //  *     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
    //  *     releaseDate: '2021-12-15T09:19:31.282Z'
    //  *   }
    //  * }
    //  */
    //
    /// Attempts to get latest release tag.
    /// ## Returns
    /// A `String` containing the latest tagged version of the launcher .
    #[inline(always)]
    pub fn get_latest_launcher_version() -> String {
        println!("Grabbing Latest launcher version...");
        // let client = reqwest::blocking::Client::new();
        "2.30.1".to_string() // Hardcode it ig...
    }

    /// Downloads the official launcher, adding this shortcut because who wants to go through the whole effort of opening the browser...
    /// ## Returns
    /// Result <(), Error>
    #[inline(always)]
    pub fn download_official_launcher(download_location: PathBuf) -> Result<(), Error> {
        println!("Downloading official launcher...");
        let client = reqwest::blocking::Client::new();
        let response = &client
            .get(
                endpoints::LAUNCHER_VERSION.to_owned()
                    + "latest/Star%20Stable%20Online%20Setup.exe",
            )
            .header("User-Agent", endpoints::USER_AGENT)
            .header("pragma", "no-cache")
            .header("cache-control", "no-cache")
            .send()
            .expect("Couldn't send GET request!")
            .bytes()
            .expect("Couldn't get raw text data!");

        let mut file = match File::create(&download_location) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };

        file.write_all(response).unwrap();
        Ok(())
    }

    /// Downloads file manifest
    /// ## Returns
    /// JsonValue with data or Error
    #[inline(always)]
    pub fn get_remote_manifest(version_hash: String) -> Result<JsonValue, Error> {
        println!("Grabbing Game status...");
        let client = reqwest::blocking::Client::new();
        match json::parse(
            &client
                .get(endpoints::GAME_FILES.to_owned() + version_hash.as_str() + "/Manifest.json")
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .header("pragma", "no-cache")
                .header("cache-control", "no-cache")
                .send()
                .expect("Couldn't send GET request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        ) {
            Ok(manifest_data) => Ok(manifest_data),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Fetches status for account bound server
    /// ## Returns
    /// structure of GameStatus containing with relevant info
    #[inline(always)]
    pub fn get_game_server_data(token: String) -> Result<GameStatus, Error> {
        println!("Grabbing Game status...");
        let client = reqwest::blocking::Client::new();
        let response = json::parse(
            &client
                .get(endpoints::GAME_SERVER_DATA.to_owned() + token.as_str())
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("Couldn't send GET request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        );

        match response {
            Ok(response) => Ok(GameStatus {
                id: response["id"].as_i16().unwrap(),
                region_id: response["regionId"].as_i8().unwrap(),
                name: response["name"].to_string(),
                friendly_name: response["friendlyName"].to_string(),
                online: response["online"].as_bool().unwrap(),
                update_in_progress: response["updateInProgress"].as_bool().unwrap(),
                icon_url: response["iconUrl"].to_string(),
                message_code: response["messageCode"].as_i16().unwrap(),
                game_version: response["gameVersion"].to_string(),
            }),
            Err(e) => Err(Error::msg(format!("Could not get game server data: {}", e))),
        }
    }

    /// Attempts to log in.
    /// ## Returns
    /// A structure containing the User ID and Launcher Hash.
    /// Errors if the API `success` value is false, or if there's an error with retrieving/sending
    /// data.
    #[inline(always)]
    pub fn login(email: String, password: String) -> Result<AuthResponse, Error> {
        let json = json::object! {
            username: email,
            password: password,
            launcherVersion: Self::get_latest_launcher_version(),
            launcherPlatform: "desktop",
            clientOsRelease: "10.0.22621",
            browserFamily: "Electron",
            deviceId: utils::get_fake_device_id()
        };

        println!("Grabbing Launcher Hash and User ID...");
        let client = reqwest::blocking::Client::new();
        let response = json::parse(
            &client
                .post(endpoints::AUTH_LOGIN)
                .body(json.dump())
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("Couldn't send POST request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        );

        match response {
            Ok(response) => {
                if response["success"].as_bool().unwrap() {
                    // Success, get the queueToken and return.
                    let launcher_hash = response["launcherHash"]
                        .as_str()
                        .expect("Couldn't find 'launcherHash'!")
                        .to_owned();

                    Ok(AuthResponse {
                        user_id: response["accountId"].to_string(),
                        launcher_hash: launcher_hash.to_owned(),
                        queue_token: Self::get_queue_token(launcher_hash, client).unwrap(),
                    })
                } else {
                    return Err(Error::msg("Could not get success data for Login request"));
                }
            }
            Err(e) => Err(Error::msg(format!(
                "Could not get response data for Login request: {}",
                e
            ))),
        }
    }

    /// Attempts to get the queue token.
    /// ## Returns
    /// A `String` containing the token.
    /// Panics if the API `success` value is `false`, or there's an error with retrieving/sending
    /// data.
    #[inline(always)]
    fn get_queue_token(
        launcher_hash: String,
        client: reqwest::blocking::Client,
    ) -> Result<String, Error> {
        println!("Grabbing Queue Token...");
        let response = json::parse(
            &client
                .post(endpoints::AUTH_QUEUE_CREATE.to_owned() + &*launcher_hash)
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("Couldn't send POST request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        );

        match response {
            Ok(response) => {
                if response["success"].as_bool().unwrap() {
                    // Success, get the queueToken and return.
                    Ok(response["queueToken"]
                        .as_str()
                        .expect("Couldn't find 'queueToken'!")
                        .to_owned())
                } else {
                    return Err(Error::msg("Couldn't get queue token"));
                }
            }
            Err(e) => Err(Error::msg(format!(
                "Couldn't get queue token response: {}",
                e
            ))),
        }
    }
}
