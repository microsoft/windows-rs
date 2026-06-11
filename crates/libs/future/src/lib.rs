#![warn(unused_qualifications)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod r#async;
#[expect(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings;
#[cfg(windows)]
#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
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
