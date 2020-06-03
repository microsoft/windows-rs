use cargo_toml::Manifest;

use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{self, Error};

pub fn run() -> error::Result<()> {
    let mut cmd = cargo();
    cmd.args(&["run"]);

    perform(&mut cmd)
}

pub fn build() -> error::Result<()> {
    let mut cmd = cargo();
    cmd.args(&["build"]);

    perform(&mut cmd)
}

fn perform(cmd: &mut Command) -> error::Result<()> {
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut o = output
        .stdout
        .expect("Child process's stdout was not configured");
    let t1 = std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        std::io::copy(&mut o, &mut stdout).unwrap();
    });
    let mut e = output
        .stderr
        .expect("Child process's stderr was not configured");
    let t2 = std::thread::spawn(move || {
        let mut stdout = std::io::stderr();
        std::io::copy(&mut e, &mut stdout).unwrap();
    });
    t1.join().unwrap();
    t2.join().unwrap();
    Ok(())
}

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
