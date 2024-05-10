use crate::api::AuthResponse;
use crate::utils::write_to_file;
use crate::UpdateArgs;
use anyhow::Error;
use json::JsonValue;
use std::fs;
use std::path::PathBuf;

const FULL_INSTALL: i8 = 0;
const UPDATE: i8 = 1;
const NON_PATCH_UPDATE: i8 = 2;
const REPAIR: i8 = 3;
const READY: i8 = 4;

/// Update or download all required game files for given patch
pub fn update_game(auth_response: AuthResponse, args: &UpdateArgs) -> Result<(), Error> {
    todo!();
    let path = &args.install_path.clone().unwrap();
    let manifest_data = get_local_manifest(path);
    let mut install_type = NON_PATCH_UPDATE;

    match manifest_data {
        Ok(data) => {
            if (data == JsonValue::Null) {
                install_type = FULL_INSTALL;
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
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
            if err.raw_os_error().unwrap() == 2 {
                return Ok(JsonValue::Null);
            }

            Err(Error::from(err))
        }
    }
}

#[inline(always)]
pub fn store_local_manifest(path: &PathBuf, data: JsonValue) -> Result<(), Error> {
    write_to_file(
        &path.clone().join("manifest.json"),
        json::stringify_pretty(data, 4),
    )
    .expect("TODO: Failed to write to manifest file");
    Ok(())
}

// exports.updateAvailable = updateAvailable;
// function getPatchFilesList(installedVersion, patches) {
// return patches.find(({ version }) => version === installedVersion);
// }
// exports.getPatchFilesList = getPatchFilesList;

// function getTotalUnpackedFileSize(files) {
// return files.reduce((total, file) => total + (file.uncompressed_size || 0), 0);
// }
// exports.getTotalUnpackedFileSize = getTotalUnpackedFileSize;
// function getLargestFileSize(files) {
// return files.reduce((largest, file) => {
// const current = file.uncompressed_size || 0;
// return current > largest ? current : largest;
// }, 0);
// }
// exports.getLargestFileSize = getLargestFileSize;
