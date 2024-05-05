use crate::api::{AuthResponse};
use crate::UpdateArgs;
use anyhow::Error;
use json::JsonValue;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Update or download all required game files for given patch
pub fn update_game(auth_response: AuthResponse, args: &UpdateArgs) -> Result<(), Error> {
    let path = &args.install_path.clone().unwrap();
    Ok(())
}

#[inline(always)]
pub fn get_local_manifest(path: PathBuf) -> Result<JsonValue, Error> {
    let contents = fs::read_to_string(path.clone().join("manifest.json"));

    match json::parse(contents.unwrap().as_str()) {
        Ok(json) => Ok(json),
        Err(_) => Err(Error::msg("Could not parse JSON data from file")),
    }
}

#[inline(always)]
pub fn store_local_manifest(path: PathBuf, data: JsonValue) -> Result<(), Error> {
    let mut file = File::create(path.clone().join("manifest.json"))?;
    file.write_all(json::stringify_pretty(data, 4).as_ref())?;
    Ok(())
}

// function updateAvailable(installedVersion, remoteManifest) {
// const match = remoteManifest.patches.find(({ version }) => version === installedVersion);
// return Boolean(match);
// }
// exports.updateAvailable = updateAvailable;
// function getPatchFilesList(installedVersion, patches) {
// return patches.find(({ version }) => version === installedVersion);
// }
// exports.getPatchFilesList = getPatchFilesList;
// function getTotalUnpackedFileSize(files) {
// return files.reduce((total, file) => total + (file.uncompressed_size || 0), 0);
// }
