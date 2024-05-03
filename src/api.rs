use crate::{endpoints, Args};

/// Implementation of the `launcher-proxy` API.
pub struct API;

/// Auth response.
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
    pub fn launch_game(args: Args) {
        let auth_response = API::login(args.email, args.password);
        if let Some(p) = args.game_path {
            if !std::path::Path::new(&p.as_path().join("SSOClient.exe")).exists() {
                panic!(
                    "[ERROR] No 'SSOClient.exe' is present. Make sure that this path is correct!"
                )
            }

            // Save me from this horrible way of passing arguments.
            std::process::Command::new(p.as_path())
                .args([
                    &format!("-Language=en"),
                    &format!("-NetworkUserId={}", auth_response.user_id),
                    "-MetricsServer=https://metrics.starstable.com/metric/v1/metrics",
                    "-MetricsGroup=[1]",
                    &format!("-LoginQueueToken={}", auth_response.queue_token),
                    &format!("-NetworkLauncherHash={}", auth_response.launcher_hash),
                    &format!(
                        "-ProjectUserDataPath=\"{}\"",
                        &p.as_path().to_string_lossy()
                    ),
                    &format!("-NetworkLauncherServer={}", endpoints::LAUNCHER_PROXY),
                ])
                .spawn()
                .expect("[ERROR] Couldn't start 'SSOClient.exe'!");
        };
    }

    /// Attempts to login.
    /// ## Returns
    /// A structure containing the User ID and Launcher Hash.
    /// Panics if the API `success` value is false, or if there's an error with retrieving/sending
    /// data.
    #[inline(always)]
    pub fn login(email: String, password: String) -> AuthResponse {
        let json = json::object! {
            username: email,
            password: password,
            launcherVersion: "2.30.1", // Update when the launcher updates.
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
                .expect("[ERROR] Couldn't send POST request!")
                .text()
                .expect("[ERROR] Couldn't get raw text response from the request!"),
        )
        .expect("[ERROR] Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("[ERROR] No 'success' key is present?")
        {
            // Success, get the queueToken and return.
            let launcher_hash = response["launcherHash"]
                .as_str()
                .expect("[ERROR] Couldn't find 'launcherHash'!")
                .to_owned();
            AuthResponse {
                user_id: response["accountId"].to_string(),
                launcher_hash: launcher_hash.to_owned(),
                queue_token: Self::get_queue_token(launcher_hash, client),
            }
        } else {
            panic!("[ERROR] Couldn't login, response: {response}")
        }
    }

    /// Attempts to get the queue token.
    /// ## Returns
    /// A `String` containing the token.
    /// Panics if the API `success` value is `false`, or there's an error with retrieving/sending
    /// data.
    #[inline(always)]
    fn get_queue_token(launcher_hash: String, client: reqwest::blocking::Client) -> String {
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
                .expect("[ERROR] Couldn't send POST request!")
                .text()
                .expect("[ERROR] Couldn't get raw text response from the request!"),
        )
        .expect("[ERROR] Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("[ERROR] No 'success' key is present?")
        {
            // Success, get the token.
            response["queueToken"]
                .as_str()
                .expect("[ERROR] Couldn't find 'queueToken'!")
                .to_owned()
        } else {
            panic!("[ERROR] Couldn't get queue token, response: {response}")
        }
    }
}
