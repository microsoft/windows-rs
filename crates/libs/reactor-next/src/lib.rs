#![doc = include_str!("../readme.md")]

#[expect(
    dead_code,
    unused_imports,
    reason = "the pure core is wired into the headless pump next"
)]
mod core;
mod element;
#[expect(
    dead_code,
    reason = "generated mounted state is wired into the headless pump next"
)]
mod generated;
#[cfg(test)]
mod native;

use generated::*;

pub use element::*;
pub use generated::public::*;
