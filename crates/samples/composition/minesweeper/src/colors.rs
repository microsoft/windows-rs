//! The named sRGB colors used by the minesweeper board. `windows-composition`
//! keeps its `Color` type minimal (just `rgb`/`rgba`), so the sample defines the
//! handful of named colors it needs here.

use windows_composition::Color;

pub const RED: Color = Color::rgb(255, 0, 0);
pub const WHITE: Color = Color::rgb(255, 255, 255);
pub const BLUE: Color = Color::rgb(0, 0, 255);
pub const ORANGE: Color = Color::rgb(255, 165, 0);
pub const LIME_GREEN: Color = Color::rgb(50, 205, 50);
pub const LIGHT_BLUE: Color = Color::rgb(173, 216, 230);
pub const LIGHT_GREEN: Color = Color::rgb(144, 238, 144);
pub const LIGHT_SALMON: Color = Color::rgb(255, 160, 122);
pub const LIGHT_STEEL_BLUE: Color = Color::rgb(176, 196, 222);
pub const MEDIUM_PURPLE: Color = Color::rgb(147, 112, 219);
pub const LIGHT_CYAN: Color = Color::rgb(224, 255, 255);
pub const MAROON: Color = Color::rgb(128, 0, 0);
pub const DARK_SEA_GREEN: Color = Color::rgb(143, 188, 143);
pub const WHITE_SMOKE: Color = Color::rgb(245, 245, 245);
pub const BLACK: Color = Color::rgb(0, 0, 0);
