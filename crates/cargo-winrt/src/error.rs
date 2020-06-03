use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No Cargo.toml could be found")]
    NoCargoToml,
    #[error("There was an error downloading the NuGet package {0}")]
    DownloadError(Box<dyn std::error::Error>),
    #[error("The Cargo.toml file was malformed")]
    MalformedManifest,
    #[error("There was some other error {0}")]
    Other(Box<dyn std::error::Error>),
}
