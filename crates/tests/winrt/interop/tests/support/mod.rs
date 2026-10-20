//! Shared by the test targets. See `readme.md`.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The client entry points and check helpers shared by all fixtures.
pub const HARNESS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/harness");

/// A fixture with a clean output directory owned by one consumer.
pub struct Fixture {
    source: PathBuf,
    /// The consumer's scratch directory for this fixture.
    pub out: PathBuf,
    /// The fixture's `api.rdl`, compiled by `build.rs`.
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

        let winmd = Path::new(env!("OUT_DIR")).join(format!("{name}.winmd"));

        Self { source, out, winmd }
    }

    /// Returns the path of an optional fixture file such as `client.cpp`.
    pub fn file(&self, name: &str) -> Option<PathBuf> {
        let path = self.source.join(name);
        path.is_file().then_some(path)
    }

    /// Returns the component DLL if the fixture has a `component.rs`.
    pub fn component(&self) -> Option<&'static Path> {
        self.file("component.rs").map(|_| component())
    }
}

/// Builds this crate's library, the component for all fixtures, and returns the DLL path.
///
/// Cargo does not give tests the path of a cdylib it builds, and where it puts the file depends on
/// the build directory layout. A nested build with its own target directory puts the DLL at a
/// known path. The first build compiles the dependencies; later builds are up-to-date checks.
fn component() -> &'static Path {
    static COMPONENT: OnceLock<PathBuf> = OnceLock::new();

    COMPONENT.get_or_init(|| {
        let target_dir = Path::new(env!("OUT_DIR")).join("component");

        let mut cargo = Command::new(env!("CARGO"));
        cargo
            .args(["build", "--quiet", "--lib", "--manifest-path"])
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
            .args(["--target", env!("TARGET"), "--target-dir"])
            .arg(&target_dir)
            .env("TEST_WINRT_INTEROP_COMPONENT", "1")
            .env("CARGO_INCREMENTAL", "0");

        run_tool(cargo);

        target_dir
            .join(env!("TARGET"))
            .join("debug")
            .join("test_winrt_interop.dll")
    })
}

/// Runs an external tool, panicking with its output if it fails.
pub fn run_tool(mut command: Command) {
    let output = command
        .output()
        .unwrap_or_else(|e| panic!("failed to start {command:?}: {e}"));

    assert!(
        output.status.success(),
        "{command:?} failed with {}\n\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
