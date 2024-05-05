use json::Error;
use crate::api::StarStableApi;
use crate::DownloadArgs;

pub fn download_launcher(download_args: &DownloadArgs) -> Result<(), Error>{
    match download_args.install_path.to_owned() {
        None => {
            let path = dirs::home_dir().unwrap().as_path().join("Star Stable Online Setup.exe");
            StarStableApi::download_official_launcher(path)
        }
        Some(path) => {
            StarStableApi::download_official_launcher(path)
        }
    }.expect("TODO: panic message");

    Ok(())

}