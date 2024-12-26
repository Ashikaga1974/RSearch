use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_specific_configuration_path: String,
}

/// Loads a configuration from a JSON file.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path to the JSON file.
///
/// # Returns
///
/// * `Result<Config, io::Error>` - On success, returns a `Config` instance containing the parsed JSON data.
///   On failure, returns an `io::Error` indicating the cause of the failure.
pub fn load_config_from_file(file_path: &str) -> Result<Config, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
