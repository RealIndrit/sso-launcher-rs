use anyhow::Error;
use rand::random;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

#[inline(always)]
pub fn write_to_file(path: &PathBuf, data: String) -> Result<(), Error> {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_ref()).unwrap();
    Ok(())
}
