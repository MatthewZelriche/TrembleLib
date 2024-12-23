use thiserror::Error;

#[derive(Error, Debug)]
#[repr(u16)]
pub enum TrembleError {
    #[error("No error occured. Used for the FFI interface")]
    Success,
    #[error("An error occured during the initialization of some resource")]
    InitError,
}
