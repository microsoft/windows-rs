//! ```compile_fail
//! use std::{marker::PhantomData, sync::MutexGuard};
//! use windows_core::*;
//! use windows_reference::IReference;
//!
//! #[derive(Clone, Default)]
//! struct Value(i32, PhantomData<MutexGuard<'static, ()>>);
//! impl imp::TypeKind for Value {
//!     type TypeKind = imp::CloneType;
//! }
//! impl RuntimeType for Value {
//!     const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Test.Value;i4)");
//! }
//! fn require_sync<T: Sync>() {}
//! require_sync::<Value>();
//! let _ = IReference::<Value>::from(Value::default());
//! ```
//!
//! ```compile_fail
//! use std::cell::Cell;
//! use windows_core::*;
//! use windows_reference::IReference;
//!
//! #[derive(Clone, Default)]
//! struct Value(Cell<i32>);
//! impl imp::TypeKind for Value {
//!     type TypeKind = imp::CloneType;
//! }
//! impl RuntimeType for Value {
//!     const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Test.Value;i4)");
//! }
//! fn require_send<T: Send>() {}
//! require_send::<Value>();
//! let _ = IReference::<Value>::from(Value::default());
//! ```

#![cfg(windows)]
#![allow(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
pub use bindings::*;
