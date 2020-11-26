use cargo_toml::{Manifest as ManifestImpl};

use std::path::PathBuf;

#[derive(Debug)]
pub struct Manifest {
    inner: ManifestImpl,
    path: PathBuf,
}

impl Manifest {
    pub(crate) fn from_slice(data: &[u8], path: PathBuf) -> anyhow::Result<Self> {
        Ok(Manifest {
            inner: ManifestImpl::from_slice(data)?,
            path,
        })
    }

    pub fn package_name(&self) -> &str {
        self.inner
            .package
            .as_ref()
            .map(|p| p.name.as_ref())
            .unwrap_or("")
    }
}
