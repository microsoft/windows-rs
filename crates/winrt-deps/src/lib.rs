pub mod cargo;
mod error;
pub mod manifest;

use std::path::PathBuf;

#[derive(Debug)]
pub enum DependencyDescriptor {
    NugetOrg { name: String, version: String },
    UrlNuget { name: String, url: String },
    LocalNuget { name: String, path: PathBuf },
    Local { name: String, path: PathBuf },
}

impl DependencyDescriptor {
    pub fn name(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { name, .. } => name,
            DependencyDescriptor::UrlNuget { name, .. } => name,
            DependencyDescriptor::LocalNuget { name, .. } => name,
            DependencyDescriptor::Local { name, .. } => name,
        }
    }

    pub fn version(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { version, .. } => version,
            DependencyDescriptor::UrlNuget { .. } => "unknown",
            DependencyDescriptor::LocalNuget { .. } => "unknown",
            DependencyDescriptor::Local { .. } => "unknown",
        }
    }
}
