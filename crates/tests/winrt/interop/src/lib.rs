//! Shared support for the interop fixture tests in `tests/`. See `readme.md`.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The default Windows metadata that fixtures reference, as a file for external tools.
pub const WINDOWS_WINMD: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../libs/default/Windows.winmd"
);

/// A fixture compiled to metadata in a clean output directory owned by one consumer.
pub struct Fixture {
    source: PathBuf,
    /// The consumer's scratch directory for this fixture.
    pub out: PathBuf,
    /// The fixture's `api.rdl` compiled by windows-rdl.
    pub winmd: PathBuf,
}

impl Fixture {
    pub fn new(consumer: &str, name: &str) -> Self {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join(name);

        let out = Path::new(env!("OUT_DIR")).join(consumer).join(name);
        if out.exists() {
            std::fs::remove_dir_all(&out).unwrap();
        }
        std::fs::create_dir_all(&out).unwrap();

        let winmd = out.join("api.winmd");
        windows_rdl::reader()
            .input(source.join("api.rdl"))
            .output(&winmd)
            .reference_default()
            .write()
            .unwrap_or_else(|e| panic!("{name}: {e}"));

        Self { source, out, winmd }
    }

    /// Returns the path of an optional fixture file such as `client.cpp`.
    pub fn file(&self, name: &str) -> Option<PathBuf> {
        let path = self.source.join(name);
        path.is_file().then_some(path)
    }
}

/// Runs an external tool, panicking with its output if it fails.
pub fn run_tool(mut command: Command) {
    let output = command
        .output()
        .unwrap_or_else(|e| panic!("failed to start {command:?}: {e}"));

    assert!(
        output.status.success(),
        "{command:?} failed\n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
