//! Diagnostics for `Interface::cast`, compiled in only when building with
//! `RUSTFLAGS=--cfg windows_cast_diagnostics`.
//!
//! Every cast is counted by call site and source/target type, including failed casts, since each
//! one costs a `QueryInterface` call. Use [`take_cast_counts`] to drain the counts.
//!
//! A cast whose `QueryInterface` returns the source pointer is reported on stderr once per call
//! site as a hint to examine. It may be an upcast that `Deref` or `.into()` would do without a
//! call, or a necessary downcast between interfaces that share a vtable through COM inheritance;
//! the two cannot be told apart at run time. The hint is skipped when the source is type-erased
//! (`IUnknown` or `IInspectable`, where a cast is the only way to recover the type) or the target
//! is `IUnknown` or `IAgileObject`, whose IUnknown-only vtables share the object's identity
//! pointer.

extern crate std;

use super::*;
use crate::{IInspectable, IUnknown, Interface};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::panic::Location;
use std::sync::Mutex;

/// The number of casts made from one call site between one source and target type.
#[derive(Clone, Debug)]
pub struct CastCount {
    pub location: &'static Location<'static>,
    pub source: &'static str,
    pub target: &'static str,
    pub count: u64,
}

type Key = (&'static str, u32, u32, &'static str, &'static str);

struct Site {
    location: &'static Location<'static>,
    count: u64,
    warned: bool,
}

static SITES: Mutex<BTreeMap<Key, Site>> = Mutex::new(BTreeMap::new());

/// Drains the cast counts recorded since the previous call, ordered by call site.
pub fn take_cast_counts() -> Vec<CastCount> {
    let mut sites = SITES.lock().unwrap();
    let counts = sites
        .iter()
        .filter(|(_, site)| site.count != 0)
        .map(|((.., source, target), site)| CastCount {
            location: site.location,
            source,
            target,
            count: site.count,
        })
        .collect();
    // Sites are kept so that a same-pointer cast is reported only once per process.
    sites.values_mut().for_each(|site| site.count = 0);
    counts
}

#[cold]
#[inline(never)]
pub(crate) fn record_cast<S: Interface, T: Interface>(
    location: &'static Location<'static>,
    same_pointer: bool,
) {
    let source = core::any::type_name::<S>();
    let target = core::any::type_name::<T>();
    let key = (
        location.file(),
        location.line(),
        location.column(),
        source,
        target,
    );
    let mut sites = SITES.lock().unwrap();
    let site = sites.entry(key).or_insert(Site {
        location,
        count: 0,
        warned: false,
    });
    site.count += 1;

    if same_pointer
        && !site.warned
        && S::IID != IUnknown::IID
        && S::IID != IInspectable::IID
        && T::IID != IUnknown::IID
        && T::IID != IAgileObject::IID
    {
        site.warned = true;
        drop(sites);
        std::eprintln!(
            "windows-core: cast::<{target}> from {source} at {location} returned the same \
             interface pointer; if {target} is a base of {source}, use Deref or .into() instead",
        );
    }
}
