#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
mod notifyicon;

pub use notifyicon::{NotifyIcon, NotifyIconBuilder, NotifyIconEvent, Point, Rect};
pub use windows_core::Result;
