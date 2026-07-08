#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]

// Compile-check the bindings generated from the in-house `metadata/win32` corpus.
// The module is produced by `build.rs` from `expected/slice.rs`, which the
// `tests/win32.rs` harness regenerates.
include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));

// #3761 / win32metadata#1044: CONTEXT layout + alignment correctness across every
// architecture. The generated slice arch-gates CONTEXT three ways; these asserts
// fail to compile if a scrape regression drops an alignment or mis-sizes a union.
// The expected sizes/alignments are clang ground truth from the pinned SDK headers
// (x64 1232/16, x86 716/4, arm64 912/16 — arm64 CONTEXT is ARM64_NT_CONTEXT). The
// x64 align comes from M128A's `align(16)` (never explicit on CONTEXT), which is the
// exact fidelity win32metadata#1044 has lacked for years.
#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(size_of::<slice::CONTEXT>() == 1232);
    assert!(align_of::<slice::CONTEXT>() == 16);
    assert!(align_of::<slice::M128A>() == 16);
};
#[cfg(target_arch = "x86")]
const _: () = {
    assert!(size_of::<slice::CONTEXT>() == 716);
    assert!(align_of::<slice::CONTEXT>() == 4);
    assert!(size_of::<slice::M128A>() == 16);
    assert!(align_of::<slice::M128A>() == 16);
};
#[cfg(target_arch = "aarch64")]
const _: () = {
    assert!(size_of::<slice::CONTEXT>() == 912);
    assert!(align_of::<slice::CONTEXT>() == 16);
    assert!(size_of::<slice::ARM64_NT_CONTEXT>() == 912);
    assert!(align_of::<slice::ARM64_NT_CONTEXT>() == 16);
};
