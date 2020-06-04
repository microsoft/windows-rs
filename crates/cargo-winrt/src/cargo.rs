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

pub fn package_manifest() -> error::Result<Manifest> {
    let bytes = std::fs::read(package_manifest_path()?).map_err(|_| Error::NoCargoToml)?;
    Manifest::from_slice(&bytes).map_err(|_| Error::MalformedManifest)
}

pub fn metadata() -> error::Result<Metadata> {
    let result = cargo()
        .args(&["metadata"])
        .output()
        .expect("Could not run `cargo metadata`");
    if !result.status.success() {
        let err = String::from_utf8_lossy(&result.stderr);
        return if err.contains("package believes it's in a workspace") {
            Err(CargoError::NotInWorkspace.into())
        } else {
            Err(Error::Other(anyhow::anyhow!("Error: {}", err).into()))
        };
    }
    let output = result.stdout;
    let value: Metadata =
        serde_json::from_slice(&output).expect("`cargo metadata` did not return json.");
    Ok(value)
}

pub fn package_manifest_path() -> error::Result<PathBuf> {
    let current = std::env::current_dir().expect("Could not find current directory");
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

pub fn workspace_target_path() -> error::Result<PathBuf> {
    Ok(metadata()?.target_directory)
}

pub fn cargo() -> Command {
    // TODO: check that cargo is installed and display nice error to user when not
    Command::new("cargo")
}

#[derive(Debug)]
pub enum CargoError {
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
