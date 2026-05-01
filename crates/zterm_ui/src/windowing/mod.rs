#[cfg(winit)]
pub mod winit;

#[cfg(target_os = "linux")]
pub use winit::WindowingSystem;
pub use zterm_ui_core::windowing::*;
