//! Bounding box module

use serde::Deserialize;

/// Bounding box
/// 
/// # Note:
/// BoundingBox contains top-left pixel and doesn't contain bottom-right pixel.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct BoundingBox {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl BoundingBox {
    /// Returns the width
    pub fn get_width(&self) -> i32 {
        self.right - self.left
    }

    /// Returns the height
    pub fn get_height(&self) -> i32 {
        self.bottom - self.top
    }
}
