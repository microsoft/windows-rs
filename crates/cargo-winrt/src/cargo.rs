use cargo_toml::Manifest;

use std::path::PathBuf;
use std::process::Command;

use crate::error::{self, Error};

pub fn workspace_manifest() -> error::Result<Manifest> {
    let bytes = std::fs::read(workspace_manifest_path()).map_err(|_| Error::NoCargoToml)?;
    Manifest::from_slice(&bytes).map_err(|_| Error::MalformedManifest)
}

pub fn workspace_root_path() -> PathBuf {
    let output = cargo()
        .args(&["metadata"])
        .output()
        .expect("Could not run `cargo metadata`")
        .stdout;
    let value: serde_json::Map<String, serde_json::Value> =
        serde_json::from_slice(&output).expect("`cargo metadata` did not return json.");
    let path = match value.get("workspace_root") {
        Some(serde_json::Value::String(s)) => s,
        _ => panic!("`cargo metadata` json was not in expected format"),
    };
    PathBuf::from(path)
}

pub fn workspace_manifest_path() -> PathBuf {
    workspace_root_path().join("Cargo.toml")
}

pub fn workspace_target_path() -> PathBuf {
    workspace_root_path().join("target")
}

pub fn cargo() -> Command {
    // TODO: check that cargo is installed and display nice error to user when not
    Command::new("cargo")
}
