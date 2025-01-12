use std::ops::Deref;

use ash::{vk, Entry};

use crate::error::TrembleError;

use super::debug::ValidationLayers;

pub struct Instance {
    instance: ash::Instance,
    validation_layers: Option<ValidationLayers>,
}

pub struct InstanceBuilder {
    min_api_version: u32,
    validation_layers: Vec<*const i8>,
    extensions: Vec<*const i8>,
}

impl InstanceBuilder {
    pub fn build(self, entry: &Entry) -> Result<Instance, TrembleError> {
        unsafe {
            let app_info = vk::ApplicationInfo {
                api_version: self.min_api_version,
                ..Default::default()
            };
            let create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                ..Default::default()
            }
            .enabled_extension_names(&self.extensions)
            .enabled_layer_names(&self.validation_layers);

            let instance = entry.create_instance(&create_info, None)?;

            let mut validation_layers = None;

            if !self.validation_layers.is_empty() {
                validation_layers = Some(ValidationLayers::new(&instance, &entry)?)
            }

            Ok(Instance {
                instance,
                validation_layers,
            })
        }
    }

    pub fn with_validation_layers(mut self, layers: Vec<*const i8>) -> Self {
        self.validation_layers.extend(layers);
        self
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.min_api_version = version;
        self
    }

    pub fn with_required_extensions(mut self, names: Vec<*const i8>) -> Self {
        self.extensions.extend(names);
        self
    }
}

impl Default for InstanceBuilder {
    fn default() -> Self {
        Self {
            min_api_version: vk::API_VERSION_1_0,
            validation_layers: Vec::new(),
            extensions: Vec::new(),
        }
    }
}

impl Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl Instance {
    pub fn builder() -> InstanceBuilder {
        InstanceBuilder::default()
    }

    pub unsafe fn destroy(&mut self) {
        self.validation_layers.as_mut().map(|x| x.destroy());
        self.instance.destroy_instance(None);
    }
}
