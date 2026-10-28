//! README examples for the umbrella crates, which disable their own doctests.
//! Other crates test their READMEs through their crate-level documentation.
//!
//! ```text
//! cargo test --doc -p test_docs
//! ```

#[doc = include_str!("../../../../libs/windows/readme.md")]
mod windows {}

#[doc = include_str!("../../../../libs/sys/readme.md")]
mod windows_sys {}
