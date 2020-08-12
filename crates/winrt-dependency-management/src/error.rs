use std::error::Error as StdError;
use thiserror::Error;

type DynError = dyn StdError + Send + Sync + 'static;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("no `Cargo.toml` could be found")]
    NoCargoToml,
    #[error("the Cargo.toml file was malformed\n\t{0}")]
    MalformedManifest(Box<DynError>),
    #[error("{0}")]
    CargoError(crate::cargo::CargoError),
    #[error("there was some unexpected IO error\n\t{0}")]
    IO(#[from] std::io::Error),
}
