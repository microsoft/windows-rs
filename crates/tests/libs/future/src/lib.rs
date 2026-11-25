//! ```compile_fail
//! use windows_core::{Error, IInspectable};
//! use windows_future::IAsyncOperation;
//! let _ = IAsyncOperation::<IInspectable>::ready(Err(Error::empty()));
//! ```
//!
//! ```compile_fail
//! use std::{marker::PhantomData, sync::MutexGuard};
//! use windows_core::*;
//! use windows_future::IAsyncOperation;
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
//! let _ = IAsyncOperation::ready(Ok(Value::default()));
//! ```
//!
//! ```compile_fail
//! use windows_core::{Error, IInspectable};
//! use windows_future::IAsyncOperation;
//! let _ = IAsyncOperation::<IInspectable>::spawn(|| Err(Error::empty()));
//! ```

#![cfg(windows)]

#[expect(nonstandard_style, clippy::missing_safety_doc)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
