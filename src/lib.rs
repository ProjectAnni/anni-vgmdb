pub mod models;
mod client;
mod utils;

pub use client::VGMClient;

#[derive(Debug, thiserror::Error)]
pub enum VGMError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error("no album found from search")]
    NoAlbumFound,

    #[error("invalid date format")]
    InvalidDate,
}

pub(crate) type Result<T> = std::result::Result<T, VGMError>;