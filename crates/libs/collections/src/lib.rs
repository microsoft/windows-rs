#![doc = include_str!("../readme.md")]
#![allow(missing_docs)]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod bindings;
pub use bindings::*;

mod bindings_impl;
use bindings_impl::*;

#[cfg(feature = "std")]
mod iterable;
#[cfg(feature = "std")]
mod map_view;
#[cfg(feature = "std")]
mod vector_view;
