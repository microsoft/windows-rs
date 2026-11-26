#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[cfg(windows)]
#[expect(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg_attr(not(feature = "std"), expect(dead_code))]
mod bindings;

#[cfg(all(windows, feature = "std"))]
mod reference;

/// A boxed reference to a value of type `T`.
///
/// Windows APIs use `Windows.Foundation.IReference<T>` to pass optional or boxed value types.
/// Use `IReference::<T>::from(value)` to box a value and [`Value`](IReference::Value) to
/// retrieve it.
#[cfg(windows)]
pub use bindings::IReference;
