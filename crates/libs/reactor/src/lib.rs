#[cfg(doctest)]
#[doc = include_str!("../readme.md")]
mod readme {}

#[cfg(doctest)]
#[doc = include_str!("../../../../docs/crates/windows-reactor.md")]
mod guide {}

#[cfg(any(test, feature = "test"))]
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

#[cfg(any(test, feature = "test"))]
pub use adapter::*;
pub use component::*;
pub use declaration::*;
pub use generated::*;
pub use ir::*;
pub use native::{App, AppCallback, AppContext, AppProxy, ScreenPoint, WindowTitleBarHeight};
#[cfg(feature = "test")]
pub use native::{
    LiveTickSubscription, bring_live_virtual_index, live_virtual_shell_counts,
    schedule_live_test_exit, subscribe_live_tick,
};
pub use reconcile::*;
pub use reference::*;
pub use window::*;
pub use windows_time::{DateTime, TimeSpan};

#[cfg(test)]
mod tests;
