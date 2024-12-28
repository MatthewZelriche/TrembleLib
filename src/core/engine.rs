use crate::{error::TrembleError, io::window_manager::WindowManager};

pub struct Engine {
    window_manager: WindowManager,
}

impl Engine {
    pub fn new() -> Result<Self, TrembleError> {
        log::info!("Completed TrembleLib initialization");
        Ok(Self {
            window_manager: WindowManager::new(true)?,
        })
    }

    pub fn tick(&mut self) -> bool {
        self.window_manager.poll()
    }

    pub fn window_manager(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
}
