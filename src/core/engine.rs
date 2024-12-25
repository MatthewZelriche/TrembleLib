#[derive(Debug)]
pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        log::info!("Completed TrembleLib initialization");
        Self {}
    }
}
