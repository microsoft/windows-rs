pub mod cargo;
mod error;
pub mod manifest;

use std::path::PathBuf;

#[derive(Debug)]
pub enum DependencyDescriptor {
    NugetOrg { name: String, version: String },
    Url { name: String, url: String },
    Local { name: String, path: PathBuf },
}

impl DependencyDescriptor {
    pub fn name(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { name, .. } => name,
            DependencyDescriptor::Url { name, .. } => name,
            DependencyDescriptor::Local { name, .. } => name,
        }
    }

    pub fn version(&self) -> &str {
        match self {
            DependencyDescriptor::NugetOrg { version, .. } => version,
            DependencyDescriptor::Url { .. } => "unknown",
            DependencyDescriptor::Local { .. } => "unknown",
        }
    }
}
