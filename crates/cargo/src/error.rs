use std::error::Error as StdError;
use thiserror::Error;

type DynError = dyn StdError + Send + Sync + 'static;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("there was an error downloading the NuGet package\n\t{0}")]
    DownloadError(Box<DynError>),
    #[error("there was some unexpected IO error\n\t{0}")]
    IO(#[from] std::io::Error),
}
