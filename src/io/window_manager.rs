use crate::error::TrembleError;
use log::{info, warn};
use send_wrapper::SendWrapper;
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    platform::{
        pump_events::{EventLoopExtPumpEvents, PumpStatus},
        windows::WindowAttributesExtWindows,
    },
    window::{Window, WindowId},
};

type WindowID = u64;

#[derive(Debug)]
pub enum CustomWindowEvents {
    CreateWindow(WindowID),
}

pub struct WindowManager {
    evt_loop: SendWrapper<EventLoop<CustomWindowEvents>>,
    evt_loop_proxy: EventLoopProxy<CustomWindowEvents>,
    app_handler: WindowAppHandler,
}

struct WindowAppHandler {
    windows: HashMap<WindowID, Option<Window>>,
    window_id_map: HashMap<WindowId, WindowID>,
    next_window_id: WindowID,
}

impl ApplicationHandler<CustomWindowEvents> for WindowAppHandler {
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {}

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: CustomWindowEvents,
    ) {
        match event {
            CustomWindowEvents::CreateWindow(id) => {
                self.create_window(id, event_loop);
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        info!("{:?}", event);

        // TODO: How to handle closing a specific window?
        match event {
            winit::event::WindowEvent::CloseRequested => self.delete_window(window_id),
            _ => (),
        }
    }
}

impl WindowAppHandler {
    pub fn should_close(&self) -> bool {
        // We should terminate the event loop if we created a main window and all windows have
        // been destroyed. The loop can also be terminated by explicitly dropping the WindowManager
        self.next_window_id > 0 && self.windows.is_empty()
    }

    fn create_window(&mut self, id: WindowID, event_loop: &ActiveEventLoop) {
        match event_loop.create_window(Window::default_attributes().with_drag_and_drop(false)) {
            Ok(window) => {
                self.window_id_map.insert(window.id(), id);
                self.windows.insert(id, Some(window)).expect("It shouldn't be possible to insert a window ID that hasn't been allocated by request_create_window");
            }
            Err(err) => {
                warn!("Failed to construct window: {}", err);
                // Delete queued entry
                self.windows.remove(&id);
            }
        }
    }

    fn delete_window(&mut self, window_id: WindowId) {
        self.windows.remove(
            &self
                .window_id_map
                .remove(&window_id)
                .expect("ID mapping should always exist"),
        );
    }
}

impl WindowManager {
    pub fn new(create_main_window: bool) -> Result<Self, TrembleError> {
        let evt_loop = SendWrapper::new(
            EventLoop::<CustomWindowEvents>::with_user_event()
                .build()
                .map_err(|e| TrembleError::InitError(e.to_string()))?,
        );
        let evt_loop_proxy = (*evt_loop).create_proxy();
        let mut this = WindowManager {
            evt_loop,
            evt_loop_proxy,
            app_handler: WindowAppHandler {
                windows: HashMap::new(),
                window_id_map: HashMap::new(),
                next_window_id: 0,
            },
        };

        if create_main_window {
            this.create_window_request()?;
        }

        Ok(this)
    }

    pub fn should_close(&self) -> bool {
        self.app_handler.should_close()
    }

    pub fn create_window_request(&mut self) -> Result<WindowID, TrembleError> {
        // TODO: Dehardcode window options
        let id = self.app_handler.next_window_id;
        // First reserve the Id in the map, before creating the window
        if self.app_handler.windows.insert(id, None).is_some() {
            panic!("It should never be possible to duplicate assign windows to the same ID!");
        }

        // Update the next ID and send the request
        self.app_handler.next_window_id += 1;
        self.evt_loop_proxy
            .send_event(CustomWindowEvents::CreateWindow(id))
            .map_err(|_| {
                TrembleError::PlatformError("Failed to send create window request!".to_string())
            })?;

        Ok(id)
    }

    pub fn poll(&mut self) -> bool {
        match (*self.evt_loop).pump_app_events(None, &mut self.app_handler) {
            PumpStatus::Continue if !self.should_close() => true,
            _ => false,
        }
    }
}
