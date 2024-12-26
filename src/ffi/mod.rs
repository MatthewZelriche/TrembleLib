use crate::{core::engine::Engine, error::TrembleError, log::initialize_logger};
use std::sync::{LazyLock, RwLock};

mod log;

static ENGINE: LazyLock<RwLock<Option<Engine>>> = LazyLock::new(|| RwLock::new(None));

#[no_mangle]
pub extern "C" fn tr_initialize() -> TrembleError {
    initialize_logger();
    let mut engine = ENGINE.write().unwrap();
    if let None = *engine {
        *engine = Some(Engine::new());
        return TrembleError::Success;
    } else {
        return TrembleError::InitError;
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
