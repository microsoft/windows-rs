use anyhow::Context;
use cargo_toml::Manifest;
use thiserror::Error;

use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{self, Error};

pub fn run() -> anyhow::Result<()> {
    let mut cmd = cargo();
    cmd.args(&["run"]);

    perform(&mut cmd)
}

pub fn build() -> anyhow::Result<()> {
    let mut cmd = cargo();
    cmd.args(&["build"]);

    perform(&mut cmd)
}

fn perform(cmd: &mut Command) -> anyhow::Result<()> {
    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let mut o = output
        .stdout
        .expect("Child process's stdout was not configured");
    let t1: std::thread::JoinHandle<anyhow::Result<()>> = std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        std::io::copy(&mut o, &mut stdout)?;
        Ok(())
    });
    let mut e = output
        .stderr
        .expect("Child process's stderr was not configured");

    let mut stdout = std::io::stderr();
    std::io::copy(&mut e, &mut stdout)?;

    t1.join().unwrap()?;
    Ok(())
}

pub fn package_manifest() -> anyhow::Result<Manifest> {
    let bytes = std::fs::read(package_manifest_path()?).map_err(|_| Error::NoCargoToml)?;
    Ok(Manifest::from_slice(&bytes).map_err(|e| Error::MalformedManifest(Box::new(e)))?)
}

pub fn metadata() -> anyhow::Result<Metadata> {
    let result = cargo()
        .args(&["metadata"])
        .output()
        .expect("Could not run `cargo metadata`");
    if !result.status.success() {
        let err = String::from_utf8_lossy(&result.stderr);
        return if err.contains("package believes it's in a workspace")
            || err.contains("could not find `Cargo.toml`")
        {
            Err(CargoError::NotInWorkspace.into())
        } else {
            anyhow::bail!("{}", err)
        };
    }
    let output = result.stdout;
    let value: Metadata =
        serde_json::from_slice(&output).expect("`cargo metadata` did not return json.");
    Ok(value)
}

pub fn package_manifest_path() -> anyhow::Result<PathBuf> {
    let _ = metadata()?;
    let current =
        std::env::current_dir().context("failed to get current directory in search of manifest")?;
    let mut current = current.as_path();
    loop {
        let manifest = current.join("Cargo.toml");
        if manifest.exists() {
            return Ok(manifest);
        }
        current = current
            .parent()
            .expect("Current directory has no parent, but it must");
    }
}

pub fn workspace_target_path() -> anyhow::Result<PathBuf> {
    Ok(metadata()?.target_directory)
}

pub fn cargo() -> Command {
    // TODO: check that cargo is installed and display nice error to user when not
    Command::new("cargo")
}

#[derive(Error, Debug)]
pub enum CargoError {
    #[error("you are not currently in cargo workspace")]
    NotInWorkspace,
}

impl std::convert::From<CargoError> for error::Error {
    fn from(cargo_error: CargoError) -> Self {
        error::Error::CargoError(cargo_error)
    }
}

#[derive(serde::Deserialize)]
pub struct Metadata {
    target_directory: PathBuf,
}
