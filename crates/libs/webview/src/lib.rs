#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
mod controller;
mod environment;
mod handler;
mod string;
mod webview;

use bindings::*;
use windows_core::*;

pub use bindings::HWND;
pub use controller::Controller;
pub use environment::{Environment, create_environment};
pub use webview::WebView;
pub use windows_core::Result;
