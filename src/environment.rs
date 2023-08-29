use crate::cell::{Cell, update_cells};

pub struct Environment {
    // attributes like resource-rich areas
    pub cells: Vec<Cell>,
}

impl Environment {
    pub fn new() -> Self {
        // Initialize environment and cells
        Self {
            // Initialize cells
        }
    }

    pub fn update(&mut self) {
        // Update the environment
        // Update cells
        update_cells(&mut self.cells);
    }
}
