use ::log::error;

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

    match Engine::new() {
        Ok(ctx) => {
            *engine = Some(ctx);
            return TrembleError::Success.into();
        }
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
}

#[no_mangle]
pub extern "C" fn tr_tick() -> bool {
    let mut lock = ENGINE.write().unwrap();
    let engine = lock.as_mut().expect("Called ffi fn before initialize()");
    engine.tick()
}

#[no_mangle]
pub extern "C" fn tr_create_window(out_id: &mut u64) -> TrembleCError {
    let mut lock = ENGINE.write().unwrap();
    let engine = lock.as_mut().expect("Called ffi fn before initialize()");
    match engine.window_manager().create_window_request() {
        Ok(id) => {
            *out_id = id;
            return TrembleError::Success.into();
        }
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
}

#[no_mangle]
pub extern "C" fn tr_shutdown() {
    ENGINE.write().unwrap().take();
    ::log::logger().flush();
}
