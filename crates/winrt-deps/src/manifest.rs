use super::DependencyDescriptor;
use crate::error::Error;

use anyhow::{anyhow, bail, Context};
use cargo_toml::{Manifest as ManifestImpl, Value};

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

    pub fn get_dependencies(self) -> anyhow::Result<Vec<DependencyDescriptor>> {
        let local_dependencies = self
            .local_dependencies()?
            .into_iter()
            .map(|d| d.get_dependencies())
            .collect::<anyhow::Result<Vec<Vec<DependencyDescriptor>>>>()?;
        let metadata = self.inner.package.and_then(|p| p.metadata);

        let mut from_metadata = match metadata {
            Some(md) => dependency_descriptors_from_metadata(md)?,
            _ => Vec::new(),
        };
        for dep in local_dependencies {
            from_metadata.extend(dep);
        }

        Ok(from_metadata)
    }

    fn local_dependencies(&self) -> anyhow::Result<Vec<Self>> {
        self.inner
            .dependencies
            .iter()
            .filter_map(|(name, dependency)| {
                let details = dependency.detail()?;
                let path = details.path.as_ref()?;
                let path = self.path.parent().unwrap().join(path);
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
                let m = Self::from_slice(&file, path)?;
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
                if t.len() > 1 {
                    bail!(Error::MalformedManifest(
                        anyhow!(
                            "expected either a `version`, `url`, `path`, or `blob` string for winrt package '{}'",
                            key
                        )
                        .into(),
                    ));
                }

                if let Some(Value::String(version)) = t.remove("version") {
                    return Ok(DependencyDescriptor::NugetOrg { name: key, version })
                }

                if let Some(Value::String(path)) = t.remove("path") {
                    return Ok(DependencyDescriptor::LocalNuget {
                        name: key,
                        path: path.into(),
                    });
                }

                if let Some(Value::String(url)) = t.remove("url") {
                    return Ok(DependencyDescriptor::UrlNuget {
                        name: key,
                        url,
                    });
                }
                if let Some(Value::String(path)) = t.remove("blob") {
                    return Ok(DependencyDescriptor::Local {
                        name: key,
                        path: path.into(),
                    });
                }

                bail!(Error::MalformedManifest(
                    anyhow!(
                        "expected either a `version`, `url`, `path`, or `blob` string for nuget package '{}'",
                        key
                    )
                    .into(),
                ));
            }
            v @ _ => bail!(Error::MalformedManifest(
                anyhow!("expected `version` as string, found {}", v).into(),
            )),
        })
        .collect()
}
