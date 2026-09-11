#![doc = include_str!("../readme.md")]

#[expect(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
mod trayicon;

pub use trayicon::{Menu, Point, Rect, TrayIcon, TrayIconBuilder, TrayIconEvent};
pub use windows_core::Result;
