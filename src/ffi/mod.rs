use std::sync::{LazyLock, RwLock};

use crate::core::engine::Engine;

static ENGINE: LazyLock<RwLock<Option<Engine>>> = LazyLock::new(|| RwLock::new(None));

#[no_mangle]
pub extern "C" fn tr_initialize() {
    let mut engine = ENGINE.write().unwrap();
    if let None = *engine {
        *engine = Some(Engine {});
    } else {
        return; // TODO: Error
    }
}

#[no_mangle]
pub extern "C" fn tr_test() {
    let lock = ENGINE.read().unwrap();
    let engine = lock.as_ref().expect("Called ffi fn before initialize()");

    engine.hello();
}

#[no_mangle]
pub extern "C" fn tr_shutdown() {
    if let None = ENGINE.write().unwrap().take() {
        return; // TODO: ERROR
    }
}
