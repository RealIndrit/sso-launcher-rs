use crate::api::StarStableApi;
use crate::DownloadArgs;
use anyhow::Error;

pub fn download_launcher(download_args: &DownloadArgs) -> Result<(), Error> {
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
