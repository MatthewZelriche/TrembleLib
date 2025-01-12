use ash::{vk, Entry};
use device::Device;
use instance::Instance;

use crate::error::TrembleError;

mod debug;
mod device;
mod instance;

pub struct Renderer {
    entry: Entry,
    instance: Instance,
}

impl Renderer {
    pub fn new() -> Result<Self, TrembleError> {
        log::info!("Initializing renderer...");

        let entry = unsafe { Entry::load()? };
        let validation_layers = if cfg!(feature = "debug_features") {
            debug::ValidationLayers::get_debug_layers(&entry)?
        } else {
            Vec::new()
        };

        let required_extensions = debug::ValidationLayers::get_debug_extensions();

        let instance = Instance::builder()
            .with_version(vk::API_VERSION_1_3)
            .with_validation_layers(validation_layers)
            .with_required_extensions(required_extensions)
            .build(&entry)?;

        let this = Self { entry, instance };

        Ok(this)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy();
        }
    }
}
