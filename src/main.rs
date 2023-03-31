//! Main script

use std::env;

use crate::config::Config;

mod debug_util;
mod error_util;
mod config;
mod bounding_box;

const CONFIG_FILE_PATH: &str = "resource/config.toml";

fn main() {
    // Load the config file.
    let config = load_config_file()
        .expect("Failed to load a config file.");
    debug!(println!("config = {:?}", config));
    println!("config = {:?}", config)
}

/// This function loads Config from a config file.
fn load_config_file() -> error_util::Result<Config> {
    let exe_file_path = env::current_exe()?;
    let exe_dir_path = exe_file_path.parent()
        .ok_or(error_util::create_error("Failed to get a parent path."))?;
    let config_file_path = exe_dir_path.join(CONFIG_FILE_PATH);
    let config = Config::from_file(&config_file_path)?;
    Ok(config)
}