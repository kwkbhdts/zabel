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
    let exe_file_path = env::current_exe()
        .expect("Failed to get the exe file path.");
    let exe_dir_path = exe_file_path.parent()
        .expect("Failed to get the exe directory path.");
    let config_file_path = exe_dir_path.join(CONFIG_FILE_PATH);
    let config = Config::from_file(&config_file_path)
        .expect("Failed to load a config file.");
    debug!(println!("config = {:?}", config));

}
