#![doc = include_str!("../readme.md")]

mod app;
#[expect(
    dead_code,
    reason = "some pure-core paths are only used by headless tests"
)]
mod core;
mod element;
#[expect(
    dead_code,
    reason = "generated mounted state is wired into the headless pump next"
)]
mod generated;
mod native;

use generated::*;

pub use app::*;
pub use core::public::*;
pub use element::*;
pub use generated::public::*;
