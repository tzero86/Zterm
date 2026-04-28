#[cfg(winit)]
pub mod winit;

pub use zterm_ui_core::windowing::*;
#[cfg(target_os = "linux")]
pub use winit::WindowingSystem;
