use ::log::error;

use crate::{
    core::engine::Engine,
    error::{TrembleCError, TrembleError},
    log::initialize_logger,
};
use std::sync::{LazyLock, RwLock};

#[repr(transparent)]
pub struct Out<T>(*mut T);

impl<T> Out<T> {
    pub fn assign(&self, val: T) {
        unsafe {
            std::ptr::write(self.0, val);
        }
    }
}

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
pub extern "C" fn tr_create_window(id: Out<u64>) -> TrembleCError {
    let mut lock = ENGINE.write().unwrap();
    let engine = lock.as_mut().expect("Called ffi fn before initialize()");
    match engine.window_manager().create_window_request() {
        Ok(new_id) => {
            id.assign(new_id);
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
