//! Diagnostics for `Interface::cast`, compiled in only when building with
//! `RUSTFLAGS=--cfg windows_cast_diagnostics`.
//!
//! Every cast is counted by call site and source/target type, including failed casts, since each
//! one costs a `QueryInterface` call. Use [`take_cast_counts`] to drain the counts.
//!
//! A cast is avoidable when the result can be had without a query whatever the object: the target
//! has the source's IID (use `as_interface`), or the target is `IInspectable` and the source derives
//! from it (use `.into()`). Casts to `IUnknown` are excluded since they return the object's
//! identity. Avoidable casts are reported on stderr once per call site and returned by
//! [`avoidable_casts`].
//!
//! A cast to a shorter vtable that returns the source pointer is reported on stderr once per call
//! site as a hint only. It is likely an upcast that `Deref` or `.into()` would do, but COM objects
//! may also share one vtable between unrelated interfaces whose layouts happen to be prefixes, so
//! it is not counted as avoidable.

extern crate std;

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
    avoidable: bool,
    hinted: bool,
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
        avoidable: false,
        hinted: false,
    });
    site.count += 1;

    let source_size = size_of::<S::Vtable>();
    let target_size = size_of::<T::Vtable>();
    let unknown_size = size_of::<<IUnknown as Interface>::Vtable>();
    let instead = if S::IID == T::IID && T::IID != IUnknown::IID {
        Some("as_interface")
    } else if S::INSPECTABLE && T::IID == IInspectable::IID {
        Some(".into()")
    } else {
        None
    };

    if site.hinted {
        return;
    }

    if let Some(instead) = instead {
        site.avoidable = true;
        site.hinted = true;
        drop(sites);
        std::eprintln!(
            "windows-core: cast::<{target}> from {source} at {location} is avoidable; use \
             {instead} instead",
        );
    } else if same_pointer && target_size > unknown_size && target_size < source_size {
        site.hinted = true;
        drop(sites);
        std::eprintln!(
            "windows-core: cast::<{target}> from {source} at {location} returned the same \
             interface pointer; if {target} is a base of {source}, use Deref or .into() instead",
        );
    }
}

/// Returns the call sites that made an avoidable cast since the process started.
pub fn avoidable_casts() -> Vec<CastCount> {
    SITES
        .lock()
        .unwrap()
        .iter()
        .filter(|(_, site)| site.avoidable)
        .map(|((.., source, target), site)| CastCount {
            location: site.location,
            source,
            target,
            count: site.count,
        })
        .collect()
}
