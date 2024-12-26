use crate::{
    core::engine::Engine,
    error::{TrembleCError, TrembleError},
    log::initialize_logger,
};
use std::sync::{LazyLock, RwLock};

mod log;

static ENGINE: LazyLock<RwLock<Option<Engine>>> = LazyLock::new(|| RwLock::new(None));

#[no_mangle]
pub extern "C" fn tr_initialize() -> TrembleCError {
    initialize_logger();
    let mut engine = ENGINE.write().unwrap();
    if let None = *engine {
        *engine = Some(Engine::new());
        return TrembleError::Success.into();
    } else {
        return TrembleError::InitError.into();
    }
}

#[no_mangle]
pub extern "C" fn tr_test() {
    let lock = ENGINE.read().unwrap();
    let engine = lock.as_ref().expect("Called ffi fn before initialize()");
}

#[no_mangle]
pub extern "C" fn tr_shutdown() {
    ENGINE.write().unwrap().take();
    ::log::logger().flush();
}
