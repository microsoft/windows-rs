#![warn(unused_qualifications)]
#![expect(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod r#async;
#[allow(unused_qualifications)]
mod bindings;
#[cfg(windows)]
mod bindings_impl;
mod join;
mod waiter;
mod when;

use r#async::*;
pub use bindings::*;
#[cfg(windows)]
use bindings_impl::*;
use waiter::*;
use windows_core::*;

#[cfg(feature = "std")]
mod async_ready;
#[cfg(feature = "std")]
mod async_spawn;
#[cfg(feature = "std")]
mod future;
