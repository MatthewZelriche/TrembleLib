use ash::LoadingError;
use strum::EnumDiscriminants;
use thiserror::Error;

#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(name(TrembleCError), repr(u16))]
pub enum TrembleError {
    #[error("No error occured. Used for the FFI interface")]
    Success,
    #[error("Engine initialization failed: {0}")]
    InitError(String),
    #[error("An error occured during interaction with the underlying platform: {0}")]
    PlatformError(String),
}

impl From<LoadingError> for TrembleError {
    fn from(value: LoadingError) -> Self {
        Self::PlatformError(value.to_string())
    }
}

impl From<ash::vk::Result> for TrembleError {
    fn from(value: ash::vk::Result) -> Self {
        Self::PlatformError(value.to_string())
    }
}
