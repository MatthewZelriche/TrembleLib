use std::borrow::Cow;
use std::ffi::CStr;

use ash::ext::debug_utils;
use ash::{vk, Entry};

use crate::error::TrembleError;

pub struct ValidationLayers {
    debug_obj: debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

#[allow(dead_code)]
impl ValidationLayers {
    const VALIDATION_NAME: &'static CStr =
        unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") };

    pub fn new(instance: &ash::Instance, entry: &Entry) -> Result<Self, TrembleError> {
        // TODO: Proper selection of message severity
        let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING
                    | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let debug_utils_loader = debug_utils::Instance::new(entry, instance);
        let callback = unsafe { debug_utils_loader.create_debug_utils_messenger(&info, None)? };

        Ok(Self {
            debug_obj: debug_utils_loader,
            debug_messenger: callback,
        })
    }

    pub fn get_debug_extensions() -> Vec<*const i8> {
        vec![vk::EXT_DEBUG_UTILS_NAME.as_ptr()]
    }

    pub fn get_debug_layers(entry: &Entry) -> Result<Vec<*const i8>, TrembleError> {
        let supported_layers = unsafe {
            entry
                .enumerate_instance_layer_properties()
                .unwrap_or_default()
        };

        let requested_layers = vec![Self::VALIDATION_NAME.as_ptr()];
        // Safety: requested_layers verified to have valid null-terminated ptrs
        unsafe { Self::layers_supported(supported_layers, &requested_layers)? };
        Ok(requested_layers)
    }

    unsafe fn layers_supported(
        supported: Vec<ash::vk::LayerProperties>,
        requested: &Vec<*const i8>,
    ) -> Result<(), TrembleError> {
        for name in requested {
            let name_cstr = std::ffi::CStr::from_ptr(*name);
            let res = supported
                .iter()
                .find(|x| x.layer_name_as_c_str().unwrap_or_default() == name_cstr);

            if let None = res {
                let name_str = name_cstr.to_string_lossy().to_string();
                return Err(TrembleError::PlatformError(format!(
                    "Vulkan layer: {name_str} not supported"
                )));
            }
        }

        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.debug_obj
            .destroy_debug_utils_messenger(self.debug_messenger, None)
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let callback_data = *p_callback_data;

    let mut message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    }
    .to_owned()
    .to_string();
    message.find(" (https://").map(|x| message.truncate(x)); // Get rid of ugly vulkan link

    log::info!("AAAA");

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!("{message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("{message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("{message}")
        }
        _ => {}
    }
    ash::vk::FALSE
}
