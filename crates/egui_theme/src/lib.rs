extern crate core;

mod typography_theme_builder;
mod theme;
mod color_theme;
mod color_variant;
mod htc;
mod tonal_palette;
mod dynamic_schema;
mod utils;
mod blend;
mod color;
mod error;
mod dislike;
mod temperature;
mod dynamic_color;
mod contrast;
mod font_theme;

#[derive(PartialEq, Eq)]
pub enum Palette {
    Primary,
    Secondary,
    Tertiary,
    Error,
    Neutral,
    NeutralVariant,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub(crate) use ahash::HashMap as Map;
pub(crate) use color_variant::*;
pub(crate) use utils::*;
pub use theme::*;
pub use font_theme::*;
pub use color_theme::{DynamicColor,ColorTheme};
pub use color::Color32OpacityExt;
