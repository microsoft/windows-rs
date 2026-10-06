#![doc = include_str!("../readme.md")]

#[cfg(doctest)]
#[doc = include_str!("../../../../docs/crates/windows-reactor.md")]
mod guide {}

#[cfg(any(test, feature = "test"))]
#[path = "test_support/adapter.rs"]
mod adapter;
mod component;
mod declaration;
mod generated;
mod ir;
#[cfg(not(feature = "test"))]
mod native;
#[cfg(feature = "test")]
pub mod native;
mod reconcile;
mod reference;
mod window;

use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::path::Path;
use std::rc::{Rc, Weak};
#[cfg(test)]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak as SyncWeak};
use std::time::Duration;

#[cfg(any(test, feature = "test"))]
pub use adapter::*;
pub use component::*;
pub use declaration::*;
pub use generated::*;
pub use ir::*;
pub use native::{App, AppCallback, AppContext, AppProxy, ScreenPoint, WindowTitleBarHeight};
#[cfg(feature = "test")]
pub use native::{
    LiveRenderingSubscription, LiveTickSubscription, bring_live_virtual_index,
    live_virtual_shell_counts, schedule_live_test_exit, subscribe_live_interval,
    subscribe_live_rendering, subscribe_live_tick,
};
pub use reconcile::*;
pub use reference::*;
pub use window::*;
pub use windows_time::{DateTime, TimeSpan};

#[cfg(test)]
mod tests;
