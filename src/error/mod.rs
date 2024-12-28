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
