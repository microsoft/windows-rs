//! ```compile_fail
//! use windows_core::{Error, IInspectable};
//! use windows_future::IAsyncOperation;
//! let _ = IAsyncOperation::<IInspectable>::ready(Err(Error::empty()));
//! ```
//!
//! ```compile_fail
//! use std::cell::Cell;
//! use windows_core::*;
//! use windows_future::IAsyncOperation;
//!
//! #[derive(Clone, Default)]
//! struct Value(Cell<i32>);
//! impl imp::TypeKind for Value {
//!     type TypeKind = imp::CloneType;
//! }
//! impl RuntimeType for Value {
//!     const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Test.Value;i4)");
//! }
//! let _ = IAsyncOperation::ready(Ok(Value::default()));
//! ```
//!
//! ```compile_fail
//! use windows_core::{Error, IInspectable};
//! use windows_future::IAsyncOperation;
//! let _ = IAsyncOperation::<IInspectable>::spawn(|| Err(Error::empty()));
//! ```

#![cfg(windows)]
