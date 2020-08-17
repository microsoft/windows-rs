use anyhow::Context;
use thiserror::Error;

use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use crate::error::{self, Error};
use crate::manifest::Manifest;

pub fn run() -> anyhow::Result<()> {
    Cargo::new()?.args(&["run"]).execute()
}

pub fn build() -> anyhow::Result<()> {
    Cargo::new()?.args(&["build"]).execute()
}

pub fn package_manifest() -> anyhow::Result<Manifest> {
    let path = package_manifest_path()?;
    let bytes = std::fs::read(&path).map_err(|_| Error::NoCargoToml)?;
    Ok(Manifest::from_slice(&bytes, path).map_err(|e| Error::MalformedManifest(e.into()))?)
}

pub(crate) fn metadata() -> anyhow::Result<Metadata> {
    let result = Cargo::new()?.args(&["metadata"]).output()?;
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

    Ok(Metadata::parse(&result.stdout))
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

pub(crate) struct Metadata {
    target_directory: PathBuf,
}

impl Metadata {
    fn parse(data: &[u8]) -> Self {
        let value: serde_json::Value =
            serde_json::from_slice(&data).expect("`cargo metadata` did not return json.");
        let td = value
            .as_object()
            .expect("`cargo metadata` was not an object");
        let string = td
            .get("target_directory")
            .and_then(|td| td.as_str())
            .expect("`cargo metadata` 'target_directory' was not a string")
            .to_owned();
        Metadata {
            target_directory: PathBuf::from(string),
        }
    }
}

struct Cargo {
    command: Command,
}

impl Cargo {
    fn new() -> anyhow::Result<Self> {
        // TODO: check that cargo is installed and display nice error to user when not

        Ok(Self {
            command: Command::new("cargo"),
        })
    }

    fn args<I: IntoIterator<Item = S>, S: AsRef<std::ffi::OsStr>>(mut self, args: I) -> Self {
        self.command.args(args);
        self
    }

    fn output(mut self) -> anyhow::Result<Output> {
        Ok(self.command.output()?)
    }

    fn execute(mut self) -> anyhow::Result<()> {
        self.command.args(&["--color", "always"]);
        let output = self
            .command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
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
}
