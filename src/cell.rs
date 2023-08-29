// Import Rayon for parallel processing
use rayon::prelude::*;
use log::debug; // Make sure to import debug

pub struct Cell {
    pub id: i32, // Added an id for example
    // your other attributes such as position, speed, etc.
}

impl Cell {
    pub fn new(id: i32) -> Self {
        // Initialize a new cell
        Self {
            id,
            // Initialize other attributes
        }
    }

    pub fn update(&mut self) {
        debug!("Updating cell with ID: {}", self.id);
        // Update the state of the cell
    }
}

// Function to update cells in parallel
pub fn update_cells(cells: &mut [Cell]) {
    cells.par_iter_mut().for_each(|cell| {
        cell.update();
    });
}
