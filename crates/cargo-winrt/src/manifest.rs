use super::DependencyDescriptor;
use crate::error::Error;

use anyhow::{anyhow, bail, Context};
use cargo_toml::{Manifest as ManifestImpl, Value};

use std::path::PathBuf;

pub(crate) struct Manifest(ManifestImpl);

impl Manifest {
    pub(crate) fn from_slice(data: &[u8]) -> anyhow::Result<Self> {
        Ok(Manifest(ManifestImpl::from_slice(data)?))
    }

    pub(crate) fn package_name(&self) -> &str {
        self.0
            .package
            .as_ref()
            .map(|p| p.name.as_ref())
            .unwrap_or("")
    }

    pub(crate) fn get_dependency_descriptors(self) -> anyhow::Result<Vec<DependencyDescriptor>> {
        let metadata = self.0.package.and_then(|p| p.metadata);

        let from_metadata = match metadata {
            Some(md) => dependency_descriptors_from_metadata(md)?,
            _ => Vec::new(),
        };

        Ok(from_metadata)
    }

    pub(crate) fn local_dependencies(&self) -> anyhow::Result<Vec<Self>> {
        self.0
            .dependencies
            .iter()
            .filter_map(|(name, dependency)| {
                let details = dependency.detail()?;
                let path = details.path.as_ref()?;
                Some((name, path))
            })
            .map(|(name, path)| {
                let mut path = PathBuf::from(path);
                path.push("Cargo.toml");
                let file = std::fs::read(&path).with_context(|| {
                    format!(
                        "failed to read dependency Cargo.toml for '{}' at '{}'",
                        name,
                        path.display()
                    )
                })?;
                let m = Self::from_slice(&file)?;
                Ok(m)
            })
            .collect()
    }
}

fn dependency_descriptors_from_metadata(
    metadata: Value,
) -> anyhow::Result<Vec<DependencyDescriptor>> {
    let mut t = match metadata {
        Value::Table(t) => t,
        _ => return Ok(Vec::new()),
    };
    let mut deps = match t.remove("winrt") {
        Some(Value::Table(deps)) => deps,
        None => return Ok(Vec::new()),
        Some(w) => bail!(Error::MalformedManifest(
            anyhow!("expected `winrt` as map, found {}", w).into(),
        )),
    };
    let deps = match deps.remove("dependencies") {
        Some(Value::Table(deps)) => deps,
        None => return Ok(Vec::new()),
        Some(d) => bail!(Error::MalformedManifest(
            anyhow!("expected `dependecies` as map, found {}", d).into(),
        )),
    };
    deps.into_iter()
        .map(|(key, value)| match value {
            Value::String(version) => Ok(DependencyDescriptor::NugetOrg { name: key, version }),
            Value::Table(mut t) => {
                let version = t.remove("version");
                let url = t.remove("url");
                let path = t.remove("path");
                match (version, url, path) {
                    (Some(Value::String(version)), None, None) => {
                        Ok(DependencyDescriptor::NugetOrg { name: key, version })
                    }
                    (None, Some(Value::String(url)), None) => Ok(DependencyDescriptor::Url {
                        name: key,
                        url,
                    }),
                    (None, None, Some(Value::String(path))) => Ok(DependencyDescriptor::Local {
                        name: key,
                        path: path.into(),
                    }),
                    _ => bail!(Error::MalformedManifest(
                        anyhow!(
                            "expected either a `version`, `url`, or `path` string for nuget package '{}'",
                            key
                        )
                        .into(),
                    )),
                }
            }
            v @ _ => bail!(Error::MalformedManifest(
                anyhow!("expected `version` as string, found {}", v).into(),
            )),
        })
        .collect()
}
