//! OQ-4 microbenchmark — QueryInterface dispatch cost, foundation vs macro.
//!
//! The OQ-4 pass criterion stated in `docs/option-d.md` is "within 1 ns/call
//! on x86_64 release builds with `lto = "thin"`" for QI(identity), QI(last),
//! and QI(unknown).
//!
//! This benchmark uses `std::time::Instant` rather than `criterion` to avoid
//! adding a dev-dep. It is *not* statistically rigorous (no warm-up phases,
//! no confidence intervals, no anti-DCE intrinsics beyond `std::hint::black_box`).
//! It produces an order-of-magnitude number sufficient to answer "do these
//! produce comparable code?" — the OQ-4 acceptance bar.
//!
//! Run with: `cargo test -p test_implement_foundation --test bench_qi
//! --release -- --nocapture --ignored bench`.
//!
//! Both implementations share the *same* `IValue` trait and the *same*
//! `Foo { x: u32 }` user type, so the only difference is the QI wiring.

use core::ffi::c_void;
use std::hint::black_box;
use std::time::Instant;
use test_implement_foundation::sample::{foundation_path, macro_path, IValue};
use windows_core::{ComObject, ComObjectInner, IUnknown, IUnknownImpl, Interface, GUID};

const ITERS: u64 = 5_000_000;
const UNKNOWN_IID: GUID = GUID::from_u128(0xdeadbeef_dead_beef_dead_beefdeadbeef);

fn bench_qi<T: IUnknownImpl>(label: &str, outer: &T, iid: &GUID, expect_ok: bool) -> f64 {
    let start = Instant::now();
    for _ in 0..ITERS {
        let mut iface: *mut c_void = core::ptr::null_mut();
        let hr = unsafe { outer.QueryInterface(black_box(iid), black_box(&mut iface)) };
        let _ = black_box(hr);
        if !iface.is_null() {
            // If we received a real interface, drop the AddRef the QI did.
            unsafe {
                drop(IUnknown::from_raw(iface));
            }
        }
    }
    let elapsed = start.elapsed();
    let ns_per_call = elapsed.as_nanos() as f64 / ITERS as f64;
    println!(
        "{:<48} {:>7.2} ns/call  ({} iters, total {:?}, expected {})",
        label,
        ns_per_call,
        ITERS,
        elapsed,
        if expect_ok { "OK" } else { "E_NOINTERFACE" },
    );
    ns_per_call
}

/// The benchmark itself. Marked `#[ignore]` so `cargo test` doesn't run it
/// in the default test invocation (it takes a few seconds and noise from
/// debug builds is misleading).
#[test]
#[ignore]
fn bench() {
    // Build two ComObjects with identical user state.
    let foundation_obj: ComObject<foundation_path::Foo> =
        foundation_path::Foo { x: 0 }.into_object();
    let macro_obj: ComObject<macro_path::Foo> = macro_path::Foo { x: 0 }.into_object();

    let foundation_outer: &foundation_path::Foo_Impl = &foundation_obj;
    let macro_outer: &macro_path::Foo_Impl = &macro_obj;

    println!();
    println!("=== OQ-4 microbenchmark: QueryInterface dispatch ===");
    println!("(release build, no LTO; this CI is Linux/non-Windows, no IMarshal path)");
    println!();
    println!("{:<48} {:>7}", "config", "ns/call");
    println!("{:-<48} {:->7}", "", "");

    // Identity IID — `IUnknown`. Both implementations short-circuit on the
    // first identity check.
    let f_id = bench_qi(
        "foundation QI(IUnknown)  [identity short-circuit]",
        foundation_outer,
        &IUnknown::IID,
        true,
    );
    let m_id = bench_qi(
        "macro      QI(IUnknown)  [identity short-circuit]",
        macro_outer,
        &IUnknown::IID,
        true,
    );

    // Last (and only) declared interface — `IValue`. Foundation scans
    // IID_SLOTS; macro emits an open-coded `if` arm.
    let f_last = bench_qi(
        "foundation QI(IValue)    [declared, slot scan]",
        foundation_outer,
        &IValue::IID,
        true,
    );
    let m_last = bench_qi(
        "macro      QI(IValue)    [declared, if-chain]",
        macro_outer,
        &IValue::IID,
        true,
    );

    // Unknown IID — falls through every check to `E_NOINTERFACE`.
    let f_unk = bench_qi(
        "foundation QI(unknown)   [worst case → E_NOINTERFACE]",
        foundation_outer,
        &UNKNOWN_IID,
        false,
    );
    let m_unk = bench_qi(
        "macro      QI(unknown)   [worst case → E_NOINTERFACE]",
        macro_outer,
        &UNKNOWN_IID,
        false,
    );

    println!();
    println!(
        "{:<48} {:>+7.2} ns",
        "delta identity (foundation − macro)",
        f_id - m_id
    );
    println!(
        "{:<48} {:>+7.2} ns",
        "delta last     (foundation − macro)",
        f_last - m_last
    );
    println!(
        "{:<48} {:>+7.2} ns",
        "delta unknown  (foundation − macro)",
        f_unk - m_unk
    );
    println!();
    println!("OQ-4 acceptance bar: ±1.00 ns/call on x86_64 release + lto=thin.");
    println!("This run is a debug-mode sanity check; the absolute numbers are not");
    println!("the OQ-4 answer. The OQ-4 answer requires a release build with LTO");
    println!("on a quiet machine. Re-run with `--release` for a real reading.");
}
