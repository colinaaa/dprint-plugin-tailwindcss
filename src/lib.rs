pub mod configuration;
mod format_text;
mod order;
mod plugin_info;
mod sort;

#[cfg(all(feature = "wasm", target_arch = "wasm32", target_os = "unknown"))]
mod wasm_plugin;

pub use format_text::{format_text, format_text_with_range};

#[cfg(all(feature = "wasm", target_arch = "wasm32", target_os = "unknown"))]
pub use wasm_plugin::*;
