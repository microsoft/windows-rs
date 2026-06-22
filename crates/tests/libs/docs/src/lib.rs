//! Compile-verification of the documentation code samples.
//!
//! Each `docs/crates/*.md` page is attached as documentation to a private module
//! so that rustdoc collects its fenced code samples as doctests. Run with:
//!
//! ```text
//! cargo test --doc -p test_docs
//! ```

#[doc = include_str!("../../../../../docs/crates/windows-animation.md")]
mod windows_animation {}

#[doc = include_str!("../../../../../docs/crates/windows-bindgen.md")]
mod windows_bindgen {}

#[doc = include_str!("../../../../../docs/crates/windows-canvas.md")]
mod windows_canvas {}

#[doc = include_str!("../../../../../docs/crates/windows-collections.md")]
mod windows_collections {}

#[doc = include_str!("../../../../../docs/crates/windows-core.md")]
mod windows_core {}

#[doc = include_str!("../../../../../docs/crates/windows-future.md")]
mod windows_future {}

#[doc = include_str!("../../../../../docs/crates/windows-implement.md")]
mod windows_implement {}

#[doc = include_str!("../../../../../docs/crates/windows-interface.md")]
mod windows_interface {}

#[doc = include_str!("../../../../../docs/crates/windows-link.md")]
mod windows_link {}

#[doc = include_str!("../../../../../docs/crates/windows-metadata.md")]
mod windows_metadata {}

#[doc = include_str!("../../../../../docs/crates/windows-numerics.md")]
mod windows_numerics {}

#[doc = include_str!("../../../../../docs/crates/windows-reactor.md")]
mod windows_reactor {}

#[doc = include_str!("../../../../../docs/crates/windows-reference.md")]
mod windows_reference {}

#[doc = include_str!("../../../../../docs/crates/windows-registry.md")]
mod windows_registry {}

#[doc = include_str!("../../../../../docs/crates/windows-result.md")]
mod windows_result {}

#[doc = include_str!("../../../../../docs/crates/windows-services.md")]
mod windows_services {}

#[doc = include_str!("../../../../../docs/crates/windows-strings.md")]
mod windows_strings {}

#[doc = include_str!("../../../../../docs/crates/windows-targets.md")]
mod windows_targets {}

#[doc = include_str!("../../../../../docs/crates/windows-threading.md")]
mod windows_threading {}

#[doc = include_str!("../../../../../docs/crates/windows-time.md")]
mod windows_time {}

#[doc = include_str!("../../../../../docs/crates/windows-version.md")]
mod windows_version {}

#[doc = include_str!("../../../../../docs/crates/windows-webview.md")]
mod windows_webview {}
