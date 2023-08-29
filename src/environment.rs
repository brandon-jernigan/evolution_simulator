use crate::cell::{Cell, update_cells};

pub struct Environment {
    // attributes like resource-rich areas
    pub cells: Vec<Cell>,
}

impl Environment {
    pub fn new() -> Self {
        // Initialize environment and cells
        let cells = vec![Cell::new(1), Cell::new(2)]; // Example cells
        Self {
            cells,
            // Initialize other attributes
        }
    }

    pub fn update(&mut self) {
        // Update the environment
        // Update cells
        update_cells(&mut self.cells);
    }
}
