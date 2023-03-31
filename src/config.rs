//! Config file module

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;

use crate::error_util;
use crate::bounding_box::BoundingBox;

/// Config data
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
    /// File path of template file (relative path from zabel.exe)
    template_file_path: String,

    /// Composition info of the template
    composition: CompositionConfig,

}

/// Composition info of the template
#[derive(Debug)]
#[derive(Deserialize)]
pub struct CompositionConfig {
    /// Number of flet number output areas
    number_of_flet_number_areas: i8,

    /// Number of guitar strings
    number_of_strings: i8,

    /// Title output area bbox (by pixel)
    title_area: BoundingBox,

    /// Flet number output areas (by pixel)
    flet_number_areas: Vec<BoundingBox>,

}

impl Config {

    /// Create Config from a config toml file.
    /// 
    /// # Params:
    /// file_path : A file path of a config toml file.
    pub fn from_file<P: AsRef<Path>>(file_path: &P) -> error_util::Result<Config> {
        let mut f = File::open(file_path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}