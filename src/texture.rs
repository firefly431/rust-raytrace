//! Module to load and process textures.
//!
//! Textures are images that are used when rendering. They represent
//! the colors in an object. Right now, it is only being used
//! for the skybox.

extern crate image;

use std::path::Path;
use color::Color;

pub type LoadError = image::ImageError;

pub fn error_description(err: LoadError) -> String {
    format!("error #{}", err)
}

/// A texture stored in memory. It can be loaded from a file, and
/// pixels can be sampled.
pub struct Texture {
    width: u32,
    height: u32,
    data: Box<[u8]>,
}

#[inline]
fn clamp(x: f64) -> f64 {
    if x < 0.0 {0.0} else if x > 1.0 {1.0} else {x}
}

impl Texture {
    /// Load a texture from a file.
    /// Assumes that the texture is in the sRGB colorspace.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Texture, LoadError> {
        let im = try!(image::open(path)).to_rgb();
        Ok(Texture { width: im.width(), height: im.height(), data: im.into_raw().into_boxed_slice() })
    }
    /// Get the color at a position. The parameters are in pixels.
    pub fn at(&self, x: u32, y: u32) -> Color {
        let idx = 3 * (x + y * self.width) as usize;
        Color::from_srgb(self.data[idx + 0], self.data[idx + 1], self.data[idx + 2])
    }
    /// Sample the color at a position. The parameters are in the
    /// range [0, 1], and they will be clamped. The result will be
    /// blended.
    pub fn sample(&self, x: f64, y: f64) -> Color {
        let x = clamp(x) * (self.width - 1) as f64;
        let y = clamp(y) * (self.height - 1) as f64;
        let x0 = x as u32;
        let x1 = if x0 >= self.width - 1 { self.width - 1 } else { x0 + 1 };
        let xx = x - (x0 as f64);
        let y0 = y as u32;
        let y1 = if y0 >= self.height - 1 { self.height - 1 } else { y0 + 1 };
        let yy = y - (y0 as f64);
        let cx0 = self.at(x0, y0) * (1.0 - yy) + self.at(x0, y1) * yy;
        let cx1 = self.at(x1, y0) * (1.0 - yy) + self.at(x1, y1) * yy;
        cx0 * (1.0 - xx) + cx1 * xx
    }
}
