use crate::{error::TrembleError, io::window_manager::WindowManager, render::Renderer};

pub struct Engine {
    window_manager: WindowManager,
    renderer: Option<Renderer>,
}

impl Engine {
    pub fn new() -> Result<Self, TrembleError> {
        log::info!("Completed TrembleLib initialization");
        Ok(Self {
            window_manager: WindowManager::new(true)?,
            renderer: Some(Renderer::new()?),
        })
    }

    pub fn tick(&mut self) -> bool {
        self.window_manager.poll()
    }

    pub fn window_manager(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
}
