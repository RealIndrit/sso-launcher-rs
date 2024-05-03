use std::fmt::Debug;
use anyhow::Error;
use crate::{endpoints, Args};

/// Implementation of the `launcher-proxy` API.
pub struct API;

/// Auth response.
#[derive(Debug)]
pub struct AuthResponse {
    /// The users Account ID.
    user_id: String,

    /// The users Launcher Hash, aka. Auth Token.
    launcher_hash: String,

    /// The queue token for the user.
    queue_token: String,
}

impl API {
    /// Launches the game using the given auth response.
    pub fn get_launch_args(args: Args) -> Result<Vec<String>, Error> {
        let path = &args.game_path.clone().unwrap();
        let exe = &path.clone().join("SSOClient.exe");
        if !std::path::Path::new(exe).exists() {
            return Err(Error::msg("No 'SSOClient.exe' is present. Make sure that this path is correct!"));
        }
        return match API::login(args.email, args.password) {
            Ok(auth_response) => {
                let launch_args = [
                    exe.display().to_string(),
                    "-Language=\"sv\"".to_string(),
                    format!("-NetworkUserId=\"{}\"", auth_response.user_id),
                    format!("-MetricsServer=\"{}\"", endpoints::METRICS),
                    format!("-MetricsGroup=\"{}\"", "[1]"),
                    format!("-LoginQueueToken=\"{}\"", auth_response.queue_token),
                    format!("-NetworkLauncherHash=\"{}\"", auth_response.launcher_hash),
                    format!("-ProjectUserDataPath=\"{}\"", &path.clone().to_string_lossy()),
                    format!("-NetworkLauncherServer=\"{}\"", endpoints::LAUNCHER_PROXY),
                ];
                Ok(launch_args.to_vec())
            },
            Err(e) => Err(e)
        };
    }


    /// When Star Stable entertainment decides to fix their shit, we will implement it here
    /// but at the time they have not updated the repo tag since 2021
    // {
    //   versionInfo: {
    //     version: '2.9.13',
    //     files: [ [Object] ],
    //     path: 'Star Stable Online Setup 2.9.13.exe',
    //     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
    //     releaseDate: '2021-12-15T09:19:31.282Z'
    //   },
    //   updateInfo: {
    //     version: '2.9.13',
    //     files: [ [Object] ],
    //     path: 'Star Stable Online Setup 2.9.13.exe',
    //     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
    //     releaseDate: '2021-12-15T09:19:31.282Z'
    //   }
    // }
    /// Attempts to get latest release tag.
    /// ## Returns
    /// A `String` containing the latest tagged version of the launcher .
    #[inline(always)]
    pub fn get_latest_launcher_version() -> String {
        return "2.30.1".to_string() // Hardcode it ig...
    }

    /// Attempts to log in.
    /// ## Returns
    /// A structure containing the User ID and Launcher Hash.
    /// Panics if the API `success` value is false, or if there's an error with retrieving/sending
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
            deviceId: "NoElectronBloatWareHereLOL"
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
        )
        .expect("Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("No 'success' key is present?")
        {
            // Success, get the queueToken and return.
            let launcher_hash = response["launcherHash"]
                .as_str()
                .expect("Couldn't find 'launcherHash'!")
                .to_owned();

            return Ok(AuthResponse {
                user_id: response["accountId"].to_string(),
                launcher_hash: launcher_hash.to_owned(),
                queue_token: Self::get_queue_token(launcher_hash, client).unwrap(),
            })
        } else {
            return Err(Error::msg("Could not get success data for Login request"))
        }
    }

    /// Attempts to get the queue token.
    /// ## Returns
    /// A `String` containing the token.
    /// Panics if the API `success` value is `false`, or there's an error with retrieving/sending
    /// data.
    #[inline(always)]
    fn get_queue_token(launcher_hash: String, client: reqwest::blocking::Client) -> Result<String, Error> {
        let json = json::object! {
            launcher_hash: launcher_hash
        };

        println!("Grabbing Queue Token...");
        let response = json::parse(
            &client
                .post(endpoints::AUTH_QUEUE_CREATE)
                .body(json.dump())
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("Couldn't send POST request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        )
        .expect("Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("No 'success' key is present?")
        {
            // Success, get the token.
            Ok(response["queueToken"]
                .as_str()
                .expect("Couldn't find 'queueToken'!")
                .to_owned())
        } else {
            return Err(Error::msg("Couldn't get queue token, response: {response}"))
        }
    }
}
