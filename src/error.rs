use std::io;
use thiserror::Error;

// pub type DynError = Box<dyn std::error::Error>;

#[derive(Error, Debug)]
pub enum GlossaError {
    #[error("L10n text cannot be found.")]
    TextNotFound,

    #[error("Failed to create arc loader.")]
    CreateArcLoader(#[source] io::Error),
}
