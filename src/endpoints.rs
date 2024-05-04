/// Login Auth back-end.
/// ## Usage
/// Uses JSON and takes these parameters:
/// `username`: String
/// `password`: String
/// `launcherVersion`: String
/// `launcherPlatform`: String, should be set to `desktop`
/// `clientOsRelease`: String, 'your os distros version number, just set to a random release version such as `10.0.22621`'
/// `browserFamily`: String, should be set to `Electron`
/// `deviceId`: String, leave empty
/// ## Output
/// A JSON Response, relevant data being `launcherHash`, `accountId`, `metricsUrl` and
/// `metricsGroups`.
///
/// Payload being sent from the bloatware launcher
/// {
///   username: 'username',
///   password: 'password',
///   deviceId: 'id',
///   launcherVersion: '2.30.1',
///   launcherPlatform: 'desktop',
///   clientOsRelease: '10.0.22621',
///   browserFamily: 'Electron'
/// }
pub const AUTH_LOGIN: &str = "https://launcher-proxy.starstable.com/launcher/auth";

/// Queue Create back-end.
/// ## Usage
/// Uses JSON and takes one parameter, which is `launcher_hash` retrieved via `AUTH_LOGIN`.
/// ## Output
/// A JSON Response, relevant data being:
/// `success`: bool
/// `passedTheQueue`: bool,
/// `queueToken`: String
pub const AUTH_QUEUE_CREATE: &str = "https://launcher-proxy.starstable.com/launcher/login-queue/";

/// Launcher Proxy URL.
pub const LAUNCHER_PROXY: &str = "https://launcher-proxy.starstable.com";

/// User Agent retrieved via `navigator.userAgent`.
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) StarStableOnline/2.18.0 Chrome/104.0.5112.124 Electron/20.3.8 Safari/537.36";

/// URL for metrics, don't touch this. Hardcoded in Electron based launcher
pub const METRICS: &str = "https://metrics.starstable.com/metric/v1/metrics";
