#![doc = include_str!("../readme.md")]

mod application;
mod component;
mod declaration;
mod generated;
mod ir;
mod native;
mod reconcile;
mod reference;
mod sealed {
    pub trait PayloadCallback<T> {}
    pub trait Sealed {}
    pub trait UnitCallback {}
}
mod window;

use std::any::*;
use std::cell::*;
use std::collections::*;
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::rc::{Weak, *};
use std::sync::atomic::*;
use std::sync::{Weak as SyncWeak, *};
use std::time::*;

pub use application::*;
pub(crate) use sealed::Sealed;

pub use component::*;
pub use declaration::*;
use generated::*;
use ir::*;
use native::*;
pub use native::{App, ScreenPoint, WindowTitleBarHeight};
use reconcile::*;
pub use reference::*;
pub use window::*;
pub use windows_time::{DateTime, TimeSpan};

#[cfg(test)]
mod tests;
