//! Hardware drivers
//! 
//! This module contains all hardware drivers including:
//! - VGA text buffer driver
//! - Keyboard driver
//! - Future drivers for other hardware

pub mod vga_buffer;
pub mod keyboard;

// Re-export commonly used items
pub use vga_buffer::*;
pub use keyboard::*;

/// Initialize all drivers
pub fn init() {
    keyboard::init();
}
