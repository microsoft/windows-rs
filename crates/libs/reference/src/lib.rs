#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[cfg(all(windows, feature = "std"))]
#[expect(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings;

#[cfg(all(windows, feature = "std"))]
mod reference;

/// A boxed reference to a value of type `T`.
///
/// Windows APIs use `Windows.Foundation.IReference<T>` to pass optional or boxed value types.
/// Use `IReference::<T>::from(value)` to box a value and [`Value`](IReference::Value) to
/// retrieve it.
#[cfg(all(windows, feature = "std"))]
pub use bindings::IReference;
