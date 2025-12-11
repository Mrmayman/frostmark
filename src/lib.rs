#![doc = include_str!("../README.md")]

mod renderer;
mod state;
mod structs;
mod style;
mod widgets;

pub use state::MarkState;
pub use structs::{ImageInfo, MarkWidget, UpdateMsg};
pub use style::Style;
