use crate::endpoints;
use anyhow::Error;
use rand::random;
use sha2::{Digest, Sha256};
use std::fmt::Debug;

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
    pub(crate) message_code: i8,

    /// current active version on the server
    pub(crate) game_version: String,
}

impl StarStableApi {
    /**
     * When Star Stable entertainment decides to fix their shit, we will implement it here
     * but at the time they have not updated the repo tag since 2021
     *
     * {
     *   versionInfo: {
     *     version: '2.9.13',
     *     files: [ [Object] ],
     *     path: 'Star Stable Online Setup 2.9.13.exe',
     *     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
     *     releaseDate: '2021-12-15T09:19:31.282Z'
     *   },
     *   updateInfo: {
     *     version: '2.9.13',
     *     files: [ [Object] ],
     *     path: 'Star Stable Online Setup 2.9.13.exe',
     *     sha512: 'EsB1RFEqivuG+w4+yulMS8BnkE8DndpwCnjbpYXD2Bg3f3f3oTU+AfMYthxx0BOH0cVIcYxlJl4I/6X0G6S4UA==',
     *     releaseDate: '2021-12-15T09:19:31.282Z'
     *   }
     * }
     */

    /// Attempts to get latest release tag.
    /// ## Returns
    /// A `String` containing the latest tagged version of the launcher .
    #[inline(always)]
    pub fn get_latest_launcher_version() -> String {
        "2.30.1".to_string() // Hardcode it ig...
    }

    /**
     * Original function used by them for deviceid is:
     * This function gets the OS native UUID/GUID asynchronously (recommended), hashed by default.
     * @param {boolean} [original=false] - If true return original value of machine id, otherwise return hashed value (sha - 256)
     */

    /// Blatantly fakes the device id, cause why the fuck do they need that for launcher?
    /// ## Returns
    /// A `String` containing TOTALLY LEGIT device id ;).

    #[inline(always)]
    pub fn get_fake_device_id() -> String {
        let mut hasher = Sha256::new();
        hasher.update(random::<f32>().to_string());
        hex::encode(hasher.finalize())
    }

    /// Downloads file manifest
    /// ## Returns
    /// JsonValue with the following structure:
    /// {
    ///     "id":91,
    ///     "regionId":8,
    ///     "name":"sso-eu-se-01",
    ///     "friendlyName":"Marshmallow Clouds",
    ///     "online":true,
    ///     "updateInProgress":false,
    ///     "iconUrl":"https://www-assets.starstable.com/ServerIcons/marshmallowcloud.png",
    ///     "messageCode":0,
    ///     "gameVersion":"SSORelease651_223278_3_PXRelease2024_15_222377_34814ad9d9a14233a919face6e9d840b"
    /// }
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
                .expect("Couldn't send POST request!")
                .text()
                .expect("Couldn't get raw text response from the request!"),
        )
        .expect("Couldn't parse response as JSON!");

        Ok(GameStatus {
            id: response["id"].as_i16().unwrap(),
            region_id: response["regionId"].as_i8().unwrap(),
            name: response["name"].to_string(),
            friendly_name: response["friendlyName"].to_string(),
            online: response["online"].as_bool().unwrap(),
            update_in_progress: response["updateInProgress"].as_bool().unwrap(),
            icon_url: response["iconUrl"].to_string(),
            message_code: response["messageCode"].as_i8().unwrap(),
            game_version: response["gameVersion"].to_string(),
        })
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
            deviceId: Self::get_fake_device_id()
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

            Ok(AuthResponse {
                user_id: response["accountId"].to_string(),
                launcher_hash: launcher_hash.to_owned(),
                queue_token: Self::get_queue_token(launcher_hash, client).unwrap(),
            })
        } else {
            Err(Error::msg("Could not get success data for Login request"))
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
            Err(Error::msg("Couldn't get queue token, response: {response}"))
        }
    }
}
