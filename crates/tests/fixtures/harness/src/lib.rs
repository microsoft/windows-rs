//! Compile-time validation of fixture golden Rust files.
//!
//! `build.rs` emits `compile_fixtures.rs` containing one module per fixture
//! that has an `expected.rs`, each `include!`ing the golden file. By
//! `include!`ing that generated file here, every `expected.rs` is compiled
//! as part of building this crate. That guarantees the goldens are not just
//! syntactically valid but also reference real items in the `windows-*`
//! crates they depend on — so a stale golden breaks the build.
//!
//! The fixture-execution test logic still lives in `tests/fixtures.rs`.

include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));
